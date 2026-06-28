use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Context;
use tokio::net::UdpSocket;
use tracing::{debug, info, warn};

fn hex_preview(data: &[u8]) -> String {
    let len = data.len().min(48);
    data[..len]
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

mod eq_protocol;
mod login_handler;
mod titanium;
mod world_handler;

use eq_protocol::packet::{self, ProtocolPacket};
use eq_protocol::session::EqSession;
use titanium::opcodes;
use titanium::structs::{self, SpawnData};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionPhase {
    Unknown,
    Login,
    World,
    Zone,
}

struct ClientState {
    phase: ConnectionPhase,
    char_name: String,
    player_spawn_id: u32,
    next_spawn_id: u32,
}

impl ClientState {
    fn new() -> Self {
        Self {
            phase: ConnectionPhase::Unknown,
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

// Track connection count per IP to detect phase
struct ConnectionTracker {
    counts: HashMap<std::net::IpAddr, u32>,
}

impl ConnectionTracker {
    fn new() -> Self {
        Self { counts: HashMap::new() }
    }

    fn next_phase(&mut self, ip: std::net::IpAddr) -> ConnectionPhase {
        let count = self.counts.entry(ip).or_insert(0);
        *count += 1;
        match *count {
            1 => ConnectionPhase::Login,
            2 => ConnectionPhase::World,
            _ => ConnectionPhase::Zone,
        }
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
    info!("Handles Login + World + Zone on single port");

    let bind_addr = "0.0.0.0:5998";
    let socket = UdpSocket::bind(bind_addr).await?;
    info!(addr = bind_addr, "UDP listener bound — waiting for EQ client");

    let mut sessions: HashMap<SocketAddr, EqSession> = HashMap::new();
    let mut client_states: HashMap<SocketAddr, ClientState> = HashMap::new();
    let mut tracker = ConnectionTracker::new();
    let mut buf = vec![0u8; 8192];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let raw = &buf[..len];

        info!(addr = %addr, len, hex = %hex_preview(raw), "UDP recv");

        if len < 2 {
            continue;
        }

        // Session request — new connection
        if raw[0] == 0x00 && raw[1] == eq_protocol::OP_SESSION_REQUEST {
            match packet::parse_protocol_packet(raw) {
                Ok(ProtocolPacket::SessionRequest {
                    protocol_version,
                    connect_code,
                    max_packet_size,
                }) => {
                    let phase = tracker.next_phase(addr.ip());
                    info!(
                        addr = %addr,
                        phase = ?phase,
                        version = protocol_version,
                        "New connection"
                    );

                    let session = EqSession::new(addr, connect_code, max_packet_size);
                    let response = packet::build_session_response(
                        connect_code,
                        session.encode_key,
                        session.crc_bytes,
                        session.max_packet_size,
                    );

                    socket.send_to(&response, addr).await?;

                    // For world phase, send proactive packets after session established
                    if phase == ConnectionPhase::World {
                        // Clone needed values before inserting session
                        let mut sess = session;
                        sessions.insert(addr, EqSession::new(addr, connect_code, max_packet_size));
                        let stored = sessions.get_mut(&addr).unwrap();
                        // Need to re-sync sequence since we created a new one
                        world_handler::send_proactive_world_packets(stored, &socket, addr).await?;
                    } else {
                        sessions.insert(addr, session);
                    }

                    let mut cs = ClientState::new();
                    cs.phase = phase;
                    client_states.insert(addr, cs);
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

        // Decode protocol layer (CRC, compression) for non-trivial packets
        if raw[0] == 0x00 {
            let proto_op = raw[1];
            match proto_op {
                eq_protocol::OP_SESSION_DISCONNECT
                | eq_protocol::OP_KEEP_ALIVE
                | eq_protocol::OP_SESSION_STAT_REQUEST
                | eq_protocol::OP_OUTBOUND_PING => {}
                _ => {
                    if !session.decode_packet(&mut raw_owned) {
                        continue;
                    }
                }
            }
        }

        match packet::parse_protocol_packet(&raw_owned) {
            Ok(ProtocolPacket::KeepAlive) => {
                socket.send_to(&packet::build_keep_alive(), addr).await?;
            }

            Ok(ProtocolPacket::SessionStatRequest { .. }) => {}

            Ok(ProtocolPacket::SessionDisconnect { .. }) => {
                let phase = client_states.get(&addr).map(|c| c.phase);
                info!(addr = %addr, phase = ?phase, "Client disconnected");
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
                    let phase = client_states.get(&addr).map(|c| c.phase).unwrap_or(ConnectionPhase::Unknown);
                    dispatch_app_packet(session, &mut client_states, addr, &socket, phase, app_opcode, app_data).await?;
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
                        let phase = client_states.get(&addr).map(|c| c.phase).unwrap_or(ConnectionPhase::Unknown);
                        dispatch_app_packet(session, &mut client_states, addr, &socket, phase, app_opcode, app_data).await?;
                    }
                }
            }

            Ok(ProtocolPacket::Combined { sub_packets }) => {
                for sub in sub_packets {
                    if sub.len() >= 4 {
                        // Combined sub-packets: first byte is the protocol opcode
                        if let Ok(ProtocolPacket::AppPacket { sequence, data }) =
                            packet::parse_protocol_packet(&[&[0x00], sub.as_slice()].concat())
                        {
                            session.process_incoming_sequence(sequence);
                            socket.send_to(&packet::build_ack(sequence), addr).await?;

                            if data.len() >= 2 {
                                let app_opcode = u16::from_le_bytes([data[0], data[1]]);
                                let app_data = &data[2..];
                                let phase = client_states.get(&addr).map(|c| c.phase).unwrap_or(ConnectionPhase::Unknown);
                                dispatch_app_packet(session, &mut client_states, addr, &socket, phase, app_opcode, app_data).await?;
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

async fn dispatch_app_packet(
    session: &mut EqSession,
    client_states: &mut HashMap<SocketAddr, ClientState>,
    addr: SocketAddr,
    socket: &UdpSocket,
    phase: ConnectionPhase,
    opcode: u16,
    data: &[u8],
) -> anyhow::Result<()> {
    match phase {
        ConnectionPhase::Login => {
            login_handler::handle_login_opcode(session, socket, addr, opcode, data).await
        }
        ConnectionPhase::World => {
            world_handler::handle_world_opcode(session, socket, addr, opcode, data).await
        }
        ConnectionPhase::Zone => {
            let cs = client_states.get_mut(&addr).unwrap();
            handle_zone_packet(session, cs, socket, addr, opcode, data).await
        }
        ConnectionPhase::Unknown => {
            // Auto-detect based on opcode
            if opcode <= 0x0021 {
                info!("Auto-detected login phase from opcode 0x{opcode:04X}");
                if let Some(cs) = client_states.get_mut(&addr) {
                    cs.phase = ConnectionPhase::Login;
                }
                login_handler::handle_login_opcode(session, socket, addr, opcode, data).await
            } else if opcode == opcodes::OP_SEND_LOGIN_INFO {
                info!("Auto-detected world phase");
                if let Some(cs) = client_states.get_mut(&addr) {
                    cs.phase = ConnectionPhase::World;
                }
                world_handler::handle_world_opcode(session, socket, addr, opcode, data).await
            } else if opcode == opcodes::OP_ZONE_ENTRY {
                info!("Auto-detected zone phase");
                if let Some(cs) = client_states.get_mut(&addr) {
                    cs.phase = ConnectionPhase::Zone;
                }
                let cs = client_states.get_mut(&addr).unwrap();
                handle_zone_packet(session, cs, socket, addr, opcode, data).await
            } else {
                debug!(opcode = format!("0x{opcode:04X}"), "Unknown phase, unhandled opcode");
                Ok(())
            }
        }
    }
}

pub async fn send_app_packet(
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

async fn handle_zone_packet(
    session: &mut EqSession,
    cs: &mut ClientState,
    socket: &UdpSocket,
    addr: SocketAddr,
    opcode: u16,
    data: &[u8],
) -> anyhow::Result<()> {
    match opcode {
        opcodes::OP_ZONE_ENTRY => {
            cs.char_name = structs::extract_zone_entry_name(data);
            cs.player_spawn_id = cs.alloc_spawn_id();

            info!(character = %cs.char_name, spawn_id = cs.player_spawn_id, "Zone: entry request");

            // 1. PlayerProfile
            let pp = structs::build_player_profile(
                &cs.char_name, 1, 1, 10, 0, -99.0, -585.0, 27.0, 52,
            );
            send_app_packet(session, socket, addr, opcodes::OP_PLAYER_PROFILE, &pp).await?;
            info!("Zone: sent PlayerProfile");

            // 2. Player spawn
            let player_spawn = structs::build_spawn_struct(&SpawnData {
                spawn_id: cs.player_spawn_id,
                name: cs.char_name.clone(), last_name: String::new(),
                level: 10, race: 1, class_id: 1, gender: 0, deity: 0,
                x: -99.0, y: -585.0, z: 27.0, heading: 0.0, size: 6.0,
                npc_type: 0, cur_hp: 100, max_hp: 100, body_type: 0,
                run_speed: 0.7, walk_speed: 0.46,
                findable: 1, light: 0, texture: 0, helm_texture: 0,
                guild_id: 0xFFFFFFFF,
            });
            send_app_packet(session, socket, addr, opcodes::OP_ZONE_ENTRY, &player_spawn).await?;

            // 3. NPC spawns
            let npcs = [
                ("Basher_Nanrum", -2.0, -567.0, 26.0),
                ("Zugor", -117.0, -603.0, 27.0),
                ("a_Troll_guard", -60.0, -600.0, 26.0),
                ("Grobb_Merchant", -130.0, -550.0, 27.0),
            ];
            for (npc_name, x, y, z) in &npcs {
                let npc_id = cs.alloc_spawn_id();
                let spawn = structs::build_spawn_struct(&SpawnData {
                    spawn_id: npc_id, name: npc_name.to_string(), last_name: String::new(),
                    level: 30, race: 9, class_id: 1, gender: 0, deity: 0,
                    x: *x, y: *y, z: *z, heading: 0.0, size: 6.0,
                    npc_type: 1, cur_hp: 100, max_hp: 100, body_type: 1,
                    run_speed: 0.7, walk_speed: 0.46,
                    findable: 1, light: 0, texture: 0, helm_texture: 0,
                    guild_id: 0xFFFFFFFF,
                });
                send_app_packet(session, socket, addr, opcodes::OP_NEW_SPAWN, &spawn).await?;
            }
            info!(count = npcs.len(), "Zone: sent NPC spawns");

            // 4. TimeOfDay + Weather
            send_app_packet(session, socket, addr, opcodes::OP_TIME_OF_DAY, &structs::build_time_of_day(14, 0, 1, 3100)).await?;
            send_app_packet(session, socket, addr, opcodes::OP_WEATHER, &structs::build_weather(0, 0)).await?;
        }

        opcodes::OP_REQ_NEW_ZONE => {
            info!("Zone: sending zone config");
            let nz = structs::build_new_zone_struct(
                &cs.char_name, "grobb", "Grobb", -99.0, -585.0, 27.0, 50.0, 800.0, 52,
            );
            send_app_packet(session, socket, addr, opcodes::OP_NEW_ZONE, &nz).await?;
        }

        opcodes::OP_REQ_CLIENT_SPAWN => {
            info!("Zone: sending ready signals");
            send_app_packet(session, socket, addr, opcodes::OP_SEND_ZONE_POINTS, &[]).await?;
            send_app_packet(session, socket, addr, opcodes::OP_SEND_EXP_ZONEIN, &[]).await?;
        }

        opcodes::OP_CLIENT_READY => {
            let sa = structs::build_spawn_appearance(cs.player_spawn_id, 0x10, cs.player_spawn_id);
            send_app_packet(session, socket, addr, opcodes::OP_SPAWN_APPEARANCE, &sa).await?;
            info!(character = %cs.char_name, "=== CLIENT IN ZONE ===");
        }

        opcodes::OP_CLIENT_UPDATE => {}
        opcodes::OP_CHANNEL_MESSAGE => {
            if data.len() > 0 {
                info!("Zone: chat ({} bytes)", data.len());
            }
        }
        opcodes::OP_SET_SERVER_FILTER => {}
        opcodes::OP_TARGET_MOUSE => {}
        opcodes::OP_CONSIDER => {}

        _ => {
            debug!(opcode = format!("0x{opcode:04X}"), len = data.len(), "Zone: unhandled");
        }
    }
    Ok(())
}
