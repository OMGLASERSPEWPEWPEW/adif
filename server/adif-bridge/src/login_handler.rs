use std::net::SocketAddr;

use des::cipher::{BlockEncryptMut, KeyIvInit};
use tokio::net::UdpSocket;
use tracing::info;

use crate::eq_protocol::session::EqSession;
use crate::titanium::opcodes;

// LoginBaseMessage: 10 bytes packed
const BASE_MSG_SIZE: usize = 10;
// LoginBaseReplyMessage: success(1) + error_str_id(4) + str_null(1) = 6 bytes
const BASE_REPLY_SIZE: usize = 6;

fn build_base_message(sequence: i32) -> Vec<u8> {
    let mut buf = vec![0u8; BASE_MSG_SIZE];
    buf[0..4].copy_from_slice(&sequence.to_le_bytes());
    buf
}

fn build_base_reply(success: bool, error_str_id: i32) -> Vec<u8> {
    let mut buf = vec![0u8; BASE_REPLY_SIZE];
    buf[0] = if success { 1 } else { 0 };
    buf[1..5].copy_from_slice(&error_str_id.to_le_bytes());
    buf[5] = 0;
    buf
}

type DesCbcEnc = cbc::Encryptor<des::Des>;

fn des_encrypt_zero_key(data: &[u8]) -> Vec<u8> {
    let key = [0u8; 8];
    let iv = [0u8; 8];

    // Pad to 8-byte alignment
    let padded_len = ((data.len() + 7) / 8) * 8;
    let mut padded = vec![0u8; padded_len];
    padded[..data.len()].copy_from_slice(data);

    let encryptor = DesCbcEnc::new(&key.into(), &iv.into());
    let encrypted = encryptor.encrypt_padded_mut::<des::cipher::block_padding::NoPadding>(&mut padded, padded_len)
        .expect("DES encryption failed");

    encrypted.to_vec()
}

pub async fn handle_login_opcode(
    session: &mut EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
    opcode: u16,
    data: &[u8],
) -> anyhow::Result<()> {
    match opcode {
        opcodes::OP_LOGIN_SESSION_READY => {
            info!("Login: session ready — sending handshake reply");
            let mut reply = build_base_message(2);
            reply.extend_from_slice(&build_base_reply(true, 101));
            crate::send_app_packet(session, socket, addr, opcodes::OP_LOGIN_CHAT_MESSAGE, &reply).await?;
        }

        opcodes::OP_LOGIN_LOGIN => {
            info!("Login: credentials received — encrypting response");

            // Extract sequence from the client's LoginBaseMessage header
            let client_sequence = if data.len() >= 4 {
                i32::from_le_bytes([data[0], data[1], data[2], data[3]])
            } else {
                3
            };

            // Part 1: LoginBaseMessage (10 bytes, PLAINTEXT)
            let mut header = vec![0u8; BASE_MSG_SIZE];
            header[0..4].copy_from_slice(&client_sequence.to_le_bytes());
            // compressed=false, encrypt_type=0, unk3=0 (already zeroed)

            // Part 2: PlayerLoginReply (will be DES-encrypted)
            // Layout: LoginBaseReplyMessage + unk1 + unk2 + lsid + key[11] + fields...
            let mut reply_data = Vec::new();
            // LoginBaseReplyMessage
            reply_data.push(1); // success = true
            reply_data.extend_from_slice(&101i32.to_le_bytes()); // error_str_id = 101 (no error)
            reply_data.push(0); // str null terminator
            // PlayerLoginReply fields
            reply_data.push(0); // unk1
            reply_data.push(0); // unk2
            reply_data.extend_from_slice(&1i32.to_le_bytes()); // lsid = 1
            // key: 11 bytes (10 chars + null)
            reply_data.extend_from_slice(b"ABCDEFGHIJ\0");
            reply_data.extend_from_slice(&0i32.to_le_bytes()); // failed_attempts
            reply_data.push(0); // show_player_count = false
            reply_data.extend_from_slice(&99i32.to_le_bytes()); // offer_min_days
            reply_data.extend_from_slice(&(-1i32).to_le_bytes()); // offer_min_views
            reply_data.extend_from_slice(&0i32.to_le_bytes()); // offer_cooldown_minutes
            reply_data.extend_from_slice(&0i32.to_le_bytes()); // web_offer_number
            reply_data.extend_from_slice(&99i32.to_le_bytes()); // web_offer_min_days
            reply_data.extend_from_slice(&(-1i32).to_le_bytes()); // web_offer_min_views
            reply_data.extend_from_slice(&0i32.to_le_bytes()); // web_offer_cooldown_minutes
            reply_data.push(0); // username null terminator
            reply_data.push(0); // unknown null terminator

            // Pad to 80 bytes (matching EQEmu's encrypted_buffer[80])
            reply_data.resize(80, 0);

            // DES CBC encrypt with zero key + zero IV
            let encrypted = des_encrypt_zero_key(&reply_data);

            // Combine: header (plaintext) + encrypted buffer
            let mut packet_data = header;
            packet_data.extend_from_slice(&encrypted);

            crate::send_app_packet(session, socket, addr, opcodes::OP_LOGIN_ACCEPTED, &packet_data).await?;
            info!("Login: sent DES-encrypted login accepted ({} bytes)", packet_data.len());

            // Immediately send server list
            send_server_list(session, socket, addr).await?;
        }

        opcodes::OP_LOGIN_SERVER_LIST_REQUEST => {
            info!("Login: server list requested");
            send_server_list(session, socket, addr).await?;
        }

        opcodes::OP_LOGIN_PLAY_REQUEST => {
            let client_sequence = if data.len() >= 4 {
                i32::from_le_bytes([data[0], data[1], data[2], data[3]])
            } else {
                0
            };
            let server_number = if data.len() >= 14 {
                u32::from_le_bytes([data[10], data[11], data[12], data[13]])
            } else {
                1
            };

            info!(sequence = client_sequence, server = server_number, "Login: play request — approving");

            let mut reply = build_base_message(client_sequence);
            reply.extend_from_slice(&build_base_reply(true, 101));
            reply.extend_from_slice(&server_number.to_le_bytes());
            crate::send_app_packet(session, socket, addr, opcodes::OP_LOGIN_PLAY_RESPONSE, &reply).await?;
            info!("Login: approved — client will reconnect for world");
        }

        _ => {
            tracing::debug!(opcode = format!("0x{opcode:04X}"), len = data.len(), "Unhandled login opcode");
        }
    }
    Ok(())
}

async fn send_server_list(
    session: &mut EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let mut buf = build_base_message(4);
    buf.extend_from_slice(&build_base_reply(true, 101));
    buf.extend_from_slice(&1i32.to_le_bytes()); // server_count

    // Per-server: null-terminated strings + i32 fields
    buf.extend_from_slice(b"127.0.0.1\0");
    buf.extend_from_slice(&1i32.to_le_bytes()); // server_type = Standard
    buf.extend_from_slice(&1u32.to_le_bytes()); // server_id
    buf.extend_from_slice(b"ADIF Dev\0");
    buf.extend_from_slice(b"us\0");
    buf.extend_from_slice(b"en\0");
    buf.extend_from_slice(&0i32.to_le_bytes()); // status = Up
    buf.extend_from_slice(&1u32.to_le_bytes()); // player_count

    crate::send_app_packet(session, socket, addr, opcodes::OP_LOGIN_SERVER_LIST_RESPONSE, &buf).await?;
    info!("Login: sent server list (1 server: ADIF Dev)");
    Ok(())
}
