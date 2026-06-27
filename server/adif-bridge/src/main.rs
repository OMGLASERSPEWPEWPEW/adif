use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Context;
use tokio::net::UdpSocket;
use tracing::{debug, info, warn};

mod eq_protocol;
mod titanium;

use eq_protocol::packet::{self, ProtocolPacket};
use eq_protocol::session::EqSession;
use titanium::opcodes;
use titanium::structs::{self, SpawnData};

struct ClientState {
    char_name: String,
    player_spawn_id: u32,
    next_spawn_id: u32,
}

impl ClientState {
    fn new() -> Self {
        Self {
            char_name: String::new(),
            player_spawn_id: 0,
            next_spawn_id: 1,
        }
    }

    fn alloc_spawn_id(&mut self) -> u32 {
        let id = self.next_spawn_id;
        self.next_spawn_id += 1;
        id
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("server.toml"));

    let config = adif_common::ServerConfig::load(&config_path)
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.server.log_level.parse().unwrap_or_default()),
        )
        .init();

    info!(name = %config.server.name, "ADIF Protocol Bridge starting");

    let bind_addr = "0.0.0.0:5998";
    let socket = UdpSocket::bind(bind_addr).await?;
    info!(addr = bind_addr, "UDP listener bound — waiting for EQ client");

    let mut sessions: HashMap<SocketAddr, EqSession> = HashMap::new();
    let mut client_states: HashMap<SocketAddr, ClientState> = HashMap::new();
    let mut buf = vec![0u8; 4096];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let raw = &buf[..len];

        if len < 2 {
            continue;
        }

        let opcode = raw[1];

        if raw[0] == 0x00 && opcode == eq_protocol::OP_SESSION_REQUEST {
            match packet::parse_protocol_packet(raw) {
                Ok(ProtocolPacket::SessionRequest {
                    protocol_version,
                    connect_code,
                    max_packet_size,
                }) => {
                    info!(
                        addr = %addr,
                        version = protocol_version,
                        "EQ client session request"
                    );

                    let session = EqSession::new(addr, connect_code, max_packet_size);
                    let response = packet::build_session_response(
                        connect_code,
                        session.encode_key,
                        session.crc_bytes,
                        session.max_packet_size,
                    );

                    socket.send_to(&response, addr).await?;
                    info!(addr = %addr, "Session established");

                    sessions.insert(addr, session);
                    client_states.insert(addr, ClientState::new());
                }
                _ => {}
            }
            continue;
        }

        let session = match sessions.get_mut(&addr) {
            Some(s) => s,
            None => continue,
        };

        let mut raw_owned = raw.to_vec();

        if raw[0] == 0x00 {
            let proto_op = raw[1];
            if proto_op != eq_protocol::OP_SESSION_DISCONNECT
                && proto_op != eq_protocol::OP_KEEP_ALIVE
                && proto_op != eq_protocol::OP_SESSION_STAT_REQUEST
                && proto_op != eq_protocol::OP_OUTBOUND_PING
            {
                if !session.decode_packet(&mut raw_owned) {
                    continue;
                }
            }
        }

        match packet::parse_protocol_packet(&raw_owned) {
            Ok(ProtocolPacket::KeepAlive) => {
                socket.send_to(&packet::build_keep_alive(), addr).await?;
            }

            Ok(ProtocolPacket::SessionStatRequest { .. }) => {}

            Ok(ProtocolPacket::SessionDisconnect { .. }) => {
                info!(addr = %addr, "Client disconnected");
                sessions.remove(&addr);
                client_states.remove(&addr);
            }

            Ok(ProtocolPacket::Ack { .. }) | Ok(ProtocolPacket::OutOfOrderAck { .. }) => {}

            Ok(ProtocolPacket::AppPacket { sequence, data }) => {
                session.process_incoming_sequence(sequence);
                socket.send_to(&packet::build_ack(sequence), addr).await?;

                if data.len() >= 2 {
                    let app_opcode = u16::from_le_bytes([data[0], data[1]]);
                    let app_data = &data[2..];
                    let cs = client_states.get_mut(&addr).unwrap();
                    handle_app_packet(session, cs, &socket, addr, app_opcode, app_data).await?;
                }
            }

            Ok(ProtocolPacket::Fragment { sequence, data }) => {
                session.process_incoming_sequence(sequence);
                socket.send_to(&packet::build_ack(sequence), addr).await?;

                let is_first = session.fragment_assembler.pending_count() == 0;
                if let Some(complete) = session.fragment_assembler.add_fragment(sequence, &data, is_first) {
                    if complete.len() >= 2 {
                        let app_opcode = u16::from_le_bytes([complete[0], complete[1]]);
                        let app_data = &complete[2..];
                        let cs = client_states.get_mut(&addr).unwrap();
                        handle_app_packet(session, cs, &socket, addr, app_opcode, app_data).await?;
                    }
                }
            }

            Ok(ProtocolPacket::Combined { sub_packets }) => {
                for sub in sub_packets {
                    if sub.len() >= 4 && sub[0] == 0x00 {
                        let mut sub_full = vec![0x00];
                        sub_full.extend_from_slice(&sub);
                        if let Ok(ProtocolPacket::AppPacket { sequence, data }) =
                            packet::parse_protocol_packet(&sub_full)
                        {
                            session.process_incoming_sequence(sequence);
                            socket.send_to(&packet::build_ack(sequence), addr).await?;

                            if data.len() >= 2 {
                                let app_opcode = u16::from_le_bytes([data[0], data[1]]);
                                let app_data = &data[2..];
                                let cs = client_states.get_mut(&addr).unwrap();
                                handle_app_packet(session, cs, &socket, addr, app_opcode, app_data).await?;
                            }
                        }
                    }
                }
            }

            Ok(ProtocolPacket::OutboundPing) => {}
            Ok(ProtocolPacket::Unknown { opcode, .. }) => {
                debug!(opcode = format!("0x{opcode:02X}"), "Unknown protocol opcode");
            }
            Err(e) => {
                debug!(error = %e, "Parse error");
            }
            _ => {}
        }
    }
}

async fn send_app_packet(
    session: &mut EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
    app_opcode: u16,
    app_data: &[u8],
) -> anyhow::Result<()> {
    let pkt = session.build_app_packet(app_opcode, app_data);
    socket.send_to(&pkt, addr).await?;
    Ok(())
}

async fn handle_app_packet(
    session: &mut EqSession,
    cs: &mut ClientState,
    socket: &UdpSocket,
    addr: SocketAddr,
    opcode: u16,
    data: &[u8],
) -> anyhow::Result<()> {
    let name = opcodes::opcode_name(opcode);

    match opcode {
        opcodes::OP_ZONE_ENTRY => {
            cs.char_name = structs::extract_zone_entry_name(data);
            cs.player_spawn_id = cs.alloc_spawn_id();

            info!(character = %cs.char_name, spawn_id = cs.player_spawn_id, "Zone entry");

            // 1. Send PlayerProfile
            let pp = structs::build_player_profile(
                &cs.char_name, 1, 1, 10, 0,
                -99.0, -585.0, 27.0, 52,
            );
            send_app_packet(session, socket, addr, opcodes::OP_PLAYER_PROFILE, &pp).await?;
            info!("Sent PlayerProfile ({} bytes)", pp.len());

            // 2. Send player's own spawn via OP_ZoneEntry (server→client)
            let player_spawn = structs::build_spawn_struct(&SpawnData {
                spawn_id: cs.player_spawn_id,
                name: cs.char_name.clone(),
                last_name: String::new(),
                level: 10, race: 1, class_id: 1, gender: 0, deity: 0,
                x: -99.0, y: -585.0, z: 27.0, heading: 0.0, size: 6.0,
                npc_type: 0, cur_hp: 100, max_hp: 100, body_type: 0,
                run_speed: 0.7, walk_speed: 0.46,
                findable: 1, light: 0, texture: 0, helm_texture: 0,
                guild_id: 0xFFFFFFFF,
            });
            send_app_packet(session, socket, addr, opcodes::OP_ZONE_ENTRY, &player_spawn).await?;
            info!("Sent player spawn");

            // 3. Send some NPC spawns
            let npcs = vec![
                ("Basher_Nanrum", 60, 2.0, -567.0, 26.0),
                ("Zugor", 62, -117.0, -603.0, 27.0),
                ("a_Troll_guard", 52, -60.0, -600.0, 26.0),
                ("Grobb_Merchant", 55, -130.0, -550.0, 27.0),
            ];
            for (npc_name, zone_id_unused, x, y, z) in &npcs {
                let npc_id = cs.alloc_spawn_id();
                let spawn = structs::build_spawn_struct(&SpawnData {
                    spawn_id: npc_id,
                    name: npc_name.to_string(),
                    last_name: String::new(),
                    level: 30, race: 9, class_id: 1, gender: 0, deity: 0,
                    x: *x, y: *y, z: *z, heading: 0.0, size: 6.0,
                    npc_type: 1, cur_hp: 100, max_hp: 100, body_type: 1,
                    run_speed: 0.7, walk_speed: 0.46,
                    findable: 1, light: 0, texture: 0, helm_texture: 0,
                    guild_id: 0xFFFFFFFF,
                });
                send_app_packet(session, socket, addr, opcodes::OP_NEW_SPAWN, &spawn).await?;
            }
            info!(count = npcs.len(), "Sent NPC spawns");

            // 4. Send TimeOfDay
            let tod = structs::build_time_of_day(14, 0, 1, 3100);
            send_app_packet(session, socket, addr, opcodes::OP_TIME_OF_DAY, &tod).await?;

            // 5. Send Weather (clear)
            let weather = structs::build_weather(0, 0);
            send_app_packet(session, socket, addr, opcodes::OP_WEATHER, &weather).await?;
        }

        opcodes::OP_REQ_NEW_ZONE => {
            info!("Client requesting zone data");
            let nz = structs::build_new_zone_struct(
                &cs.char_name, "grobb", "Grobb",
                -99.0, -585.0, 27.0, 50.0, 800.0, 52,
            );
            send_app_packet(session, socket, addr, opcodes::OP_NEW_ZONE, &nz).await?;
            info!("Sent NewZone ({} bytes)", nz.len());
        }

        opcodes::OP_REQ_CLIENT_SPAWN => {
            info!("Client requesting spawns — sending zone ready signals");

            // Send empty zone points
            send_app_packet(session, socket, addr, opcodes::OP_SEND_ZONE_POINTS, &[]).await?;
            // Send exp zonein
            send_app_packet(session, socket, addr, opcodes::OP_SEND_EXP_ZONEIN, &[]).await?;

            info!("Sent zone ready signals");
        }

        opcodes::OP_CLIENT_READY => {
            info!("Client ready — finalizing zone entry");

            // Send SpawnAppearance with player's spawn ID (type=0x10 SpawnID)
            let sa = structs::build_spawn_appearance(cs.player_spawn_id, 0x10, cs.player_spawn_id);
            send_app_packet(session, socket, addr, opcodes::OP_SPAWN_APPEARANCE, &sa).await?;

            info!(character = %cs.char_name, "CLIENT IN ZONE — zone entry complete!");
        }

        opcodes::OP_CLIENT_UPDATE => {
            debug!("Position update ({} bytes)", data.len());
        }

        opcodes::OP_CHANNEL_MESSAGE => {
            if data.len() > 0 {
                info!("Chat message ({} bytes)", data.len());
            }
        }

        opcodes::OP_SET_SERVER_FILTER => {}
        opcodes::OP_TARGET_MOUSE => {}
        opcodes::OP_CONSIDER => {}

        _ => {
            debug!(
                opcode = format!("0x{opcode:04X}"),
                name,
                len = data.len(),
                "Unhandled app opcode"
            );
        }
    }

    Ok(())
}
