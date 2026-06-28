use std::net::SocketAddr;

use tokio::net::UdpSocket;
use tracing::info;

use crate::eq_protocol::session::EqSession;
use crate::titanium::opcodes;

pub async fn send_proactive_world_packets(
    session: &mut EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    // GuildsList — empty
    crate::send_app_packet(session, socket, addr, opcodes::OP_GUILDS_LIST, &[0u8; 4]).await?;

    // ApproveWorld — 544-byte struct (mostly zeroed is fine)
    crate::send_app_packet(session, socket, addr, opcodes::OP_APPROVE_WORLD, &[0u8; 544]).await?;

    info!("World: sent proactive packets (GuildsList + ApproveWorld)");
    Ok(())
}

pub async fn handle_world_opcode(
    session: &mut EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
    opcode: u16,
    data: &[u8],
) -> anyhow::Result<()> {
    match opcode {
        opcodes::OP_SEND_LOGIN_INFO => {
            let account_name = extract_account_name(data);
            info!(account = %account_name, "World: login info received");

            // Send LogServer
            let mut log_server = vec![0u8; 128];
            let name = b"ADIF";
            log_server[..name.len()].copy_from_slice(name);
            crate::send_app_packet(session, socket, addr, opcodes::OP_LOG_SERVER, &log_server).await?;

            // Send MOTD
            let motd = b"Welcome to ADIF - Another Day In Forever\0";
            crate::send_app_packet(session, socket, addr, opcodes::OP_MOTD, motd).await?;

            // Send EnterWorld (auto-login char name)
            let char_name = b"Ghouldan\0";
            crate::send_app_packet(session, socket, addr, opcodes::OP_ENTER_WORLD, char_name).await?;

            // Send PostEnterWorld (empty)
            crate::send_app_packet(session, socket, addr, opcodes::OP_POST_ENTER_WORLD, &[]).await?;

            // Send ExpansionInfo (all expansions)
            crate::send_app_packet(session, socket, addr, opcodes::OP_EXPANSION_INFO, &0x7FFFu32.to_le_bytes()).await?;

            // Send character list
            send_character_list(session, socket, addr).await?;

            info!("World: sent character list");
        }

        opcodes::OP_ENTER_WORLD => {
            let char_name = extract_enter_world_name(data);
            info!(character = %char_name, "World: entering world");

            // Send ZoneServerInfo pointing to ourselves
            let mut zsi = vec![0u8; 130];
            let ip = b"127.0.0.1";
            zsi[..ip.len()].copy_from_slice(ip);
            let port: u16 = 5998;
            zsi[128] = (port & 0xFF) as u8;
            zsi[129] = (port >> 8) as u8;
            crate::send_app_packet(session, socket, addr, opcodes::OP_ZONE_SERVER_INFO, &zsi).await?;

            info!("World: sent zone server info (127.0.0.1:5998) — client will reconnect for zone");
        }

        _ => {
            tracing::debug!(opcode = format!("0x{opcode:04X}"), "Unhandled world opcode");
        }
    }
    Ok(())
}

fn extract_account_name(data: &[u8]) -> String {
    // LoginInfo struct: first 64 bytes contain "account_id\0password"
    if data.is_empty() {
        return String::from("Unknown");
    }
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len().min(64));
    String::from_utf8_lossy(&data[..end]).to_string()
}

fn extract_enter_world_name(data: &[u8]) -> String {
    // EnterWorld_Struct: first 64 bytes are character name
    if data.is_empty() {
        return String::from("Unknown");
    }
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len().min(64));
    String::from_utf8_lossy(&data[..end]).to_string()
}

async fn send_character_list(
    session: &mut EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    // CharacterSelect_Struct for Titanium:
    // The format is a serialized blob. For Titanium, the struct is relatively simple:
    // Each character entry contains name, class, race, level, zone, etc.
    //
    // For MVP, send a single hardcoded character: Ghouldan, level 10, Human Warrior, in Grobb

    // Titanium CharacterSelect is a flat packed struct with fixed-size entries
    // Total struct: CharCount(u32) + TotalChars(u32) + entries[]
    // Each entry is roughly 320 bytes with name[64], class, race, level, zone, etc.

    let mut buf = Vec::new();

    // CharCount = 1
    buf.extend_from_slice(&1u32.to_le_bytes());
    // TotalChars = 8 (max allowed)
    buf.extend_from_slice(&8u32.to_le_bytes());

    // Character entry (simplified — pad to expected size)
    let mut entry = vec![0u8; 320];

    // Name at offset 0 (64 bytes)
    let name = b"Ghouldan";
    entry[..name.len()].copy_from_slice(name);

    // Class at offset 64 (u8)
    entry[64] = 1; // Warrior

    // Race at offset 68 (u32)
    entry[68] = 1; // Human

    // Level at offset 72 (u8)
    entry[72] = 10;

    // Zone at offset 76 (u32) — zoneidnumber
    entry[76] = 52; // Grobb

    // Gender at offset 80
    entry[80] = 0; // Male

    // Face at offset 84
    entry[84] = 1;

    buf.extend_from_slice(&entry);

    crate::send_app_packet(session, socket, addr, opcodes::OP_SEND_CHAR_INFO, &buf).await?;
    Ok(())
}
