use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

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
use titanium::structs::{self, PlayerProfileData, SpawnData};

use adif_world::WorldState;

const LOGIN_PORT: u16 = 5998;
const WORLD_PORT: u16 = 9000;
const ZONE_PORT: u16 = 7778;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionPhase {
    Login,
    World,
    Zone,
}

struct ClientState {
    phase: ConnectionPhase,
    account_id: Option<i32>,
    account_name: String,
    char_name: String,
    char_zone_id: Option<i32>,
    player_spawn_id: u32,
    next_spawn_id: u32,
}

impl ClientState {
    fn new(phase: ConnectionPhase) -> Self {
        Self {
            phase,
            account_id: None,
            account_name: String::new(),
            char_name: String::new(),
            char_zone_id: None,
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

struct PhaseState {
    sessions: HashMap<SocketAddr, EqSession>,
    client_states: HashMap<SocketAddr, ClientState>,
}

impl PhaseState {
    fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            client_states: HashMap::new(),
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

    let pool = adif_common::create_pool(&config.database)
        .await
        .context("Failed to connect to PostgreSQL")?;
    info!("Connected to PostgreSQL");

    let world_state = Arc::new(WorldState::new(
        pool,
        config.server.name.clone(),
        "Welcome to ADIF - Another Day In Forever".to_string(),
    ));

    {
        let mut registry = world_state.zone_registry.write().await;
        let zone_addr = SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            ZONE_PORT,
        );
        registry.register(52, "grobb".to_string(), zone_addr);
        registry.register(46, "innothule".to_string(), zone_addr);
    }

    let login_socket = UdpSocket::bind(format!("0.0.0.0:{LOGIN_PORT}")).await?;
    let world_socket = UdpSocket::bind(format!("0.0.0.0:{WORLD_PORT}")).await?;
    let zone_socket = UdpSocket::bind(format!("0.0.0.0:{ZONE_PORT}")).await?;

    info!(login = LOGIN_PORT, world = WORLD_PORT, zone = ZONE_PORT, "UDP listeners bound");

    let mut login_state = PhaseState::new();
    let mut world_state_phase = PhaseState::new();
    let mut zone_state = PhaseState::new();

    let mut login_buf = vec![0u8; 8192];
    let mut world_buf = vec![0u8; 8192];
    let mut zone_buf = vec![0u8; 8192];

    loop {
        tokio::select! {
            result = login_socket.recv_from(&mut login_buf) => {
                let (len, addr) = result?;
                handle_udp_packet(
                    &login_buf[..len], addr, ConnectionPhase::Login,
                    &mut login_state, &login_socket, &world_state,
                ).await?;
            }
            result = world_socket.recv_from(&mut world_buf) => {
                let (len, addr) = result?;
                handle_udp_packet(
                    &world_buf[..len], addr, ConnectionPhase::World,
                    &mut world_state_phase, &world_socket, &world_state,
                ).await?;
            }
            result = zone_socket.recv_from(&mut zone_buf) => {
                let (len, addr) = result?;
                handle_udp_packet(
                    &zone_buf[..len], addr, ConnectionPhase::Zone,
                    &mut zone_state, &zone_socket, &world_state,
                ).await?;
            }
        }
    }
}

async fn handle_udp_packet(
    raw: &[u8],
    addr: SocketAddr,
    phase: ConnectionPhase,
    state: &mut PhaseState,
    socket: &UdpSocket,
    world_state: &Arc<WorldState>,
) -> anyhow::Result<()> {
    let len = raw.len();

    debug!(addr = %addr, len, phase = ?phase, hex = %hex_preview(raw), "UDP recv");

    if len < 2 {
        return Ok(());
    }

    if raw[0] == 0x00 && raw[1] == eq_protocol::OP_SESSION_REQUEST {
        match packet::parse_protocol_packet(raw) {
            Ok(ProtocolPacket::SessionRequest {
                protocol_version,
                connect_code,
                max_packet_size,
            }) => {
                info!(
                    addr = %addr,
                    phase = ?phase,
                    version = protocol_version,
                    "New {:?} connection", phase
                );

                let crc_bytes = if phase == ConnectionPhase::Login { 0 } else { 2 };
                let session = EqSession::new(addr, connect_code, max_packet_size, crc_bytes);
                let response = packet::build_session_response(
                    connect_code,
                    session.encode_key,
                    session.crc_bytes,
                    session.max_packet_size,
                );

                socket.send_to(&response, addr).await?;

                state.sessions.insert(addr, session);

                state.client_states.insert(addr, ClientState::new(phase));
            }
            _ => {}
        }
        return Ok(());
    }

    let session = match state.sessions.get_mut(&addr) {
        Some(s) => s,
        None => return Ok(()),
    };

    let mut raw_owned = raw.to_vec();

    // CRC decode: only SessionRequest/SessionResponse/OutOfSession are exempt (per EQEmu PacketCanBeEncoded)
    if raw[0] == 0x00 {
        let proto_op = raw[1];
        match proto_op {
            eq_protocol::OP_SESSION_REQUEST | eq_protocol::OP_SESSION_RESPONSE | eq_protocol::OP_OUT_OF_SESSION => {}
            _ => {
                if !session.decode_packet(&mut raw_owned) {
                    return Ok(());
                }
            }
        }
    }

    match packet::parse_protocol_packet(&raw_owned) {
        Ok(ProtocolPacket::KeepAlive) => {
            send_proto_packet(session, socket, addr, &packet::build_keep_alive()).await?;
        }

        Ok(ProtocolPacket::SessionStatRequest { .. }) => {}

        Ok(ProtocolPacket::SessionDisconnect { .. }) => {
            info!(addr = %addr, phase = ?phase, "Client disconnected");
            state.sessions.remove(&addr);
            state.client_states.remove(&addr);
        }

        Ok(ProtocolPacket::Ack { .. }) | Ok(ProtocolPacket::OutOfOrderAck { .. }) => {}

        Ok(ProtocolPacket::AppPacket { sequence, data }) => {
            session.process_incoming_sequence(sequence);
            send_proto_packet(session, socket, addr, &packet::build_ack(sequence)).await?;

            if data.len() >= 2 {
                let app_opcode = u16::from_le_bytes([data[0], data[1]]);
                let app_data = &data[2..];
                dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
            }
        }

        Ok(ProtocolPacket::Fragment { sequence, data }) => {
            session.process_incoming_sequence(sequence);
            send_proto_packet(session, socket, addr, &packet::build_ack(sequence)).await?;

            let is_first = session.fragment_assembler.pending_count() == 0;
            if let Some(complete) = session.fragment_assembler.add_fragment(sequence, &data, is_first) {
                if complete.len() >= 2 {
                    let app_opcode = u16::from_le_bytes([complete[0], complete[1]]);
                    let app_data = &complete[2..];
                    dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
                }
            }
        }

        Ok(ProtocolPacket::Combined { sub_packets }) => {
            for sub in sub_packets {
                if sub.len() >= 2 {
                    let full = if sub[0] == 0x00 {
                        sub.clone()
                    } else {
                        [&[0x00], sub.as_slice()].concat()
                    };
                    match packet::parse_protocol_packet(&full) {
                        Ok(ProtocolPacket::AppPacket { sequence, data }) => {
                            session.process_incoming_sequence(sequence);
                            send_proto_packet(session, socket, addr, &packet::build_ack(sequence)).await?;
                            if data.len() >= 2 {
                                let app_opcode = u16::from_le_bytes([data[0], data[1]]);
                                let app_data = &data[2..];
                                dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
                            }
                        }
                        Ok(ProtocolPacket::Fragment { sequence, data }) => {
                            session.process_incoming_sequence(sequence);
                            send_proto_packet(session, socket, addr, &packet::build_ack(sequence)).await?;
                            let is_first = session.fragment_assembler.pending_count() == 0;
                            if let Some(complete) = session.fragment_assembler.add_fragment(sequence, &data, is_first) {
                                if complete.len() >= 2 {
                                    let app_opcode = u16::from_le_bytes([complete[0], complete[1]]);
                                    let app_data = &complete[2..];
                                    dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
                                }
                            }
                        }
                        Ok(ProtocolPacket::Ack { .. }) | Ok(ProtocolPacket::OutOfOrderAck { .. }) => {}
                        _ => {}
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

    Ok(())
}

async fn dispatch_app_packet(
    session: &mut EqSession,
    client_states: &mut HashMap<SocketAddr, ClientState>,
    addr: SocketAddr,
    socket: &UdpSocket,
    phase: ConnectionPhase,
    opcode: u16,
    data: &[u8],
    world_state: &Arc<WorldState>,
) -> anyhow::Result<()> {
    match phase {
        ConnectionPhase::Login => {
            login_handler::handle_login_opcode(session, socket, addr, opcode, data).await
        }
        ConnectionPhase::World => {
            let cs = client_states.get_mut(&addr).unwrap();
            world_handler::handle_world_opcode(session, cs, socket, addr, opcode, data, world_state).await
        }
        ConnectionPhase::Zone => {
            let cs = client_states.get_mut(&addr).unwrap();
            handle_zone_packet(session, cs, socket, addr, opcode, data, world_state).await
        }
    }
}

async fn send_proto_packet(
    session: &EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
    data: &[u8],
) -> anyhow::Result<()> {
    let mut buf = data.to_vec();
    eq_protocol::codec::append_crc(&mut buf, session.encode_key, session.crc_bytes);
    socket.send_to(&buf, addr).await?;
    Ok(())
}

pub async fn send_app_packet(
    session: &mut EqSession,
    socket: &UdpSocket,
    addr: SocketAddr,
    app_opcode: u16,
    app_data: &[u8],
) -> anyhow::Result<()> {
    let pkt = session.build_app_packet(app_opcode, app_data);
    debug!(opcode = format!("0x{app_opcode:04X}"), app_bytes = app_data.len(), udp_bytes = pkt.len(), "TX");
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
    world_state: &Arc<WorldState>,
) -> anyhow::Result<()> {
    match opcode {
        opcodes::OP_ZONE_ENTRY => {
            cs.char_name = structs::extract_zone_entry_name(data);
            cs.player_spawn_id = cs.alloc_spawn_id();

            info!(character = %cs.char_name, spawn_id = cs.player_spawn_id, "Zone: entry request");

            let record = adif_world::character::load_character_by_name(
                &world_state.pool,
                &cs.char_name,
            ).await?;

            let ppd = if let Some(ref r) = record {
                PlayerProfileData {
                    name: r.name.clone(), last_name: r.last_name.clone(),
                    race: r.race as u32, class_id: r.class_id as u32,
                    level: r.level as u8, gender: r.gender as u32,
                    deity: r.deity as u32,
                    x: r.x, y: r.y, z: r.z, heading: r.heading,
                    zone_id: r.zone_id as u16,
                    face: r.face as u8, hair_color: r.hair_color as u8,
                    beard_color: r.beard_color as u8,
                    eye_color_1: r.eye_color_1 as u8, eye_color_2: r.eye_color_2 as u8,
                    hair_style: r.hair_style as u8, beard: r.beard as u8,
                }
            } else {
                warn!(character = %cs.char_name, "Zone: character not found in DB, using defaults");
                PlayerProfileData {
                    name: cs.char_name.clone(), last_name: String::new(),
                    race: 9, class_id: 1, level: 1, gender: 0, deity: 0,
                    x: -99.0, y: -585.0, z: 27.0, heading: 0.0,
                    zone_id: 52, face: 0, hair_color: 0, beard_color: 0,
                    eye_color_1: 0, eye_color_2: 0, hair_style: 0, beard: 0,
                }
            };

            let race = ppd.race;
            let class_id = ppd.class_id;
            let level = ppd.level;
            let gender = ppd.gender;
            let deity = ppd.deity as u16;
            let x = ppd.x;
            let y = ppd.y;
            let z = ppd.z;
            let heading = ppd.heading;
            let last_name = ppd.last_name.clone();

            let pp = structs::build_player_profile_full(&ppd);
            send_app_packet(session, socket, addr, opcodes::OP_PLAYER_PROFILE, &pp).await?;
            info!(race, class_id, level, "Zone: sent PlayerProfile from DB");

            let size = match race {
                1 => 6.0, 2 => 6.0, 3 => 8.0, 4 => 5.0, 5 => 4.0,
                6 => 5.0, 7 => 5.0, 8 => 7.0, 9 => 8.0, 10 => 7.0,
                11 => 6.0, 12 => 6.0, 128 => 5.0, 130 => 5.0, _ => 6.0,
            };

            let player_spawn = structs::build_spawn_struct(&SpawnData {
                spawn_id: cs.player_spawn_id,
                name: cs.char_name.clone(), last_name,
                level, race, class_id: class_id as u8, gender: gender as u8, deity,
                x, y, z, heading, size,
                npc_type: 0, cur_hp: 100, max_hp: 100, body_type: 0,
                run_speed: 0.7, walk_speed: 0.46,
                findable: 1, light: 0, texture: 0, helm_texture: 0,
                guild_id: 0xFFFFFFFF,
            });
            send_app_packet(session, socket, addr, opcodes::OP_ZONE_ENTRY, &player_spawn).await?;

            let npcs = [
                ("Basher_Nanrum", -2.0, -567.0, 26.0),
                ("Zugor", -117.0, -603.0, 27.0),
                ("a_Troll_guard", -60.0, -600.0, 26.0),
                ("Grobb_Merchant", -130.0, -550.0, 27.0),
            ];
            for (npc_name, nx, ny, nz) in &npcs {
                let npc_id = cs.alloc_spawn_id();
                let spawn = structs::build_spawn_struct(&SpawnData {
                    spawn_id: npc_id, name: npc_name.to_string(), last_name: String::new(),
                    level: 30, race: 9, class_id: 1, gender: 0, deity: 0,
                    x: *nx, y: *ny, z: *nz, heading: 0.0, size: 8.0,
                    npc_type: 1, cur_hp: 100, max_hp: 100, body_type: 1,
                    run_speed: 0.7, walk_speed: 0.46,
                    findable: 1, light: 0, texture: 0, helm_texture: 0,
                    guild_id: 0xFFFFFFFF,
                });
                send_app_packet(session, socket, addr, opcodes::OP_NEW_SPAWN, &spawn).await?;
            }
            info!(count = npcs.len(), "Zone: sent NPC spawns");

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
            if !data.is_empty() {
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
