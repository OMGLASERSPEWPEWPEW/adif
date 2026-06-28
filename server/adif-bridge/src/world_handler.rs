use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::UdpSocket;
use tracing::{info, warn};

use crate::eq_protocol::session::EqSession;
use crate::titanium::opcodes;
use crate::ClientState;

use adif_world::WorldState;

pub async fn handle_world_opcode(
    session: &mut EqSession,
    cs: &mut ClientState,
    socket: &UdpSocket,
    addr: SocketAddr,
    opcode: u16,
    data: &[u8],
    world_state: &Arc<WorldState>,
) -> anyhow::Result<()> {
    match opcode {
        opcodes::OP_SEND_LOGIN_INFO => {
            let login_info = extract_account_name(data);
            info!(login_info = %login_info, "World: login info received");

            let account = if let Ok(id) = login_info.parse::<i32>() {
                adif_world::account::find_account_by_id(&world_state.pool, id).await?
            } else {
                adif_world::account::find_account_by_name(&world_state.pool, &login_info).await?
            };

            let account = match account {
                Some(a) => a,
                None => {
                    warn!(login_info = %login_info, "World: account not found");
                    return Ok(());
                }
            };

            if account.is_banned {
                warn!(account_id = account.id, "World: account is banned");
                return Ok(());
            }

            cs.account_id = Some(account.id);
            cs.account_name = account.name.clone();
            info!(account_id = account.id, status = account.status, "World: account authenticated");

            // Character list from DB (query first so we can use it for EnterWorld)
            let chars = adif_world::character::load_character_list(
                &world_state.pool,
                account.id,
            )
            .await?;

            // EQEmu packet order: GuildsList, LogServer, EnterWorld, PostEnterWorld, ExpansionInfo, SendCharInfo

            // GuildsList (empty — 4 bytes of zeros = 0 guilds)
            crate::send_app_packet(session, socket, addr, opcodes::OP_GUILDS_LIST, &[0u8; 4]).await?;

            // LogServer (264 bytes — worldshortname at offset 32)
            let mut log_server = vec![0u8; 264];
            let sname = world_state.server_name.as_bytes();
            let slen = sname.len().min(31);
            log_server[32..32 + slen].copy_from_slice(&sname[..slen]);
            crate::send_app_packet(session, socket, addr, opcodes::OP_LOG_SERVER, &log_server).await?;

            // EnterWorld (always sent — auto-login character name or empty)
            let enter_name = if !chars.is_empty() {
                let mut n = chars[0].name.as_bytes().to_vec();
                n.push(0);
                n
            } else {
                vec![0u8]
            };
            crate::send_app_packet(session, socket, addr, opcodes::OP_ENTER_WORLD, &enter_name).await?;

            // PostEnterWorld
            crate::send_app_packet(session, socket, addr, opcodes::OP_POST_ENTER_WORLD, &[]).await?;

            // ExpansionInfo (all expansions)
            crate::send_app_packet(session, socket, addr, opcodes::OP_EXPANSION_INFO, &0x7FFFu32.to_le_bytes()).await?;

            // Character list
            let char_buf = build_titanium_char_select(&chars);
            crate::send_app_packet(session, socket, addr, opcodes::OP_SEND_CHAR_INFO, &char_buf).await?;

            info!(characters = chars.len(), "World: sent character list from DB");
        }

        opcodes::OP_ENTER_WORLD => {
            let char_name = extract_enter_world_name(data);
            info!(character = %char_name, "World: entering world");

            let account_id = cs.account_id.unwrap_or(0);

            if let Some(record) = adif_world::character::load_character(
                &world_state.pool,
                account_id,
                &char_name,
            )
            .await?
            {
                cs.char_name = record.name.clone();
                cs.char_zone_id = Some(record.zone_id);

                info!(
                    character = %record.name,
                    zone_id = record.zone_id,
                    level = record.level,
                    "World: character loaded from DB"
                );

                // MOTD (sent before ZoneServerInfo per EQEmu capture)
                let motd = b"Welcome to ADIF - Another Day In Forever\0";
                crate::send_app_packet(session, socket, addr, opcodes::OP_MOTD, motd).await?;

                let zsi = adif_world::zone_routing::build_zone_server_info_bytes(
                    &adif_world::zone_routing::ZoneRouteInfo {
                        ip: "127.0.0.1".to_string(),
                        port: crate::ZONE_PORT,
                        zone_id: record.zone_id,
                        zone_short_name: String::new(),
                    },
                );
                crate::send_app_packet(session, socket, addr, opcodes::OP_ZONE_SERVER_INFO, &zsi).await?;

                info!(port = crate::ZONE_PORT, "World: sent zone server info — client will reconnect for zone");
            } else {
                warn!(character = %char_name, account_id, "World: character not found or not owned");
            }
        }

        opcodes::OP_APPROVE_WORLD => {
            info!("World: client approved world");
        }

        _ => {
            tracing::debug!(opcode = format!("0x{opcode:04X}"), "Unhandled world opcode");
        }
    }
    Ok(())
}

fn extract_account_name(data: &[u8]) -> String {
    if data.is_empty() {
        return String::from("Unknown");
    }
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len().min(64));
    String::from_utf8_lossy(&data[..end]).to_string()
}

fn extract_enter_world_name(data: &[u8]) -> String {
    if data.is_empty() {
        return String::from("Unknown");
    }
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len().min(64));
    String::from_utf8_lossy(&data[..end]).to_string()
}

fn build_titanium_char_select(chars: &[adif_world::character::CharSelectEntry]) -> Vec<u8> {
    // Titanium CharacterSelect_Struct: 1704 bytes, parallel arrays for 10 slots
    // Offsets from titanium_structs.h (pragma pack 1)
    let mut buf = vec![0u8; 1704];

    for i in 0..10usize {
        buf[820 + i] = 0xFF; // Unknown820 — always 0xFF
        buf[902 + i] = 0xFF; // Unknown902 — always 0xFF

        if let Some(ch) = chars.get(i) {
            let race = (ch.race as u32).min(474);

            // Race[i] — offset 0, u32[10]
            buf[i * 4..i * 4 + 4].copy_from_slice(&race.to_le_bytes());

            // CS_Colors[i] — offset 40, TintProfile[10] (36 bytes each) — all zero = no tint
            // (equipment colors stay zeroed for now — naked character)

            // BeardColor[i] — offset 400, u8[10]
            buf[400 + i] = ch.beard_color as u8;
            // HairStyle[i] — offset 410, u8[10]
            buf[410 + i] = ch.hair_style as u8;

            // Equip[i] — offset 420, TextureProfile[10] (36 bytes each) — all zero = naked
            // (equipment materials stay zeroed for now)

            // SecondaryIDFile[i] — offset 780, u32[10] — 0 = no secondary weapon
            // PrimaryIDFile[i] — offset 912, u32[10] — 0 = no primary weapon

            // Deity[i] — offset 832, u32[10]
            buf[832 + i * 4..836 + i * 4].copy_from_slice(&(ch.deity as u32).to_le_bytes());
            // GoHome[i] — offset 872, u8[10]
            buf[872 + i] = 1;
            // Tutorial[i] — offset 882, u8[10]
            buf[882 + i] = 0;
            // Beard[i] — offset 892, u8[10]
            buf[892 + i] = ch.beard as u8;

            // HairColor[i] — offset 952, u8[10]
            buf[952 + i] = ch.hair_color as u8;

            // Zone[i] — offset 964, u32[10]
            buf[964 + i * 4..968 + i * 4].copy_from_slice(&(ch.zone_id as u32).to_le_bytes());
            // Class[i] — offset 1004, u8[10]
            buf[1004 + i] = ch.class_id as u8;
            // Face[i] — offset 1014, u8[10]
            buf[1014 + i] = ch.face as u8;

            // Name[i] — offset 1024, char[10][64]
            let name_off = 1024 + i * 64;
            let name_bytes = ch.name.as_bytes();
            let name_len = name_bytes.len().min(63);
            buf[name_off..name_off + name_len].copy_from_slice(&name_bytes[..name_len]);

            // Gender[i] — offset 1664, u8[10]
            buf[1664 + i] = ch.gender as u8;
            // EyeColor1[i] — offset 1674, u8[10]
            buf[1674 + i] = ch.eye_color_1 as u8;
            // EyeColor2[i] — offset 1684, u8[10]
            buf[1684 + i] = ch.eye_color_2 as u8;
            // Level[i] — offset 1694, u8[10]
            buf[1694 + i] = ch.level as u8;
        } else {
            let name_off = 1024 + i * 64;
            buf[name_off..name_off + 6].copy_from_slice(b"<none>");
        }
    }

    buf
}
