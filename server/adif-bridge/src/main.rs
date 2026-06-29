use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;
use sqlx;
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
use titanium::structs::{self, PlayerProfileData, SpawnData, ZoneData};

use adif_world::WorldState;

#[derive(Debug, sqlx::FromRow)]
struct ZoneDbRow {
    short_name: String,
    long_name: String,
    safe_x: f32,
    safe_y: f32,
    safe_z: f32,
    minclip: f32,
    maxclip: f32,
    fog_minclip: f32,
    fog_maxclip: f32,
    fog_minclip2: f32,
    fog_maxclip2: f32,
    fog_minclip3: f32,
    fog_maxclip3: f32,
    fog_minclip4: f32,
    fog_maxclip4: f32,
    fog_red: i16,
    fog_green: i16,
    fog_blue: i16,
    fog_red2: i16,
    fog_green2: i16,
    fog_blue2: i16,
    fog_red3: i16,
    fog_green3: i16,
    fog_blue3: i16,
    fog_red4: i16,
    fog_green4: i16,
    fog_blue4: i16,
    fog_density: f32,
    sky: i16,
    ztype: i16,
    zone_exp_multiplier: f32,
    gravity: f32,
    time_type: i16,
    rain_chance1: i32,
    rain_chance2: i32,
    rain_chance3: i32,
    rain_chance4: i32,
    rain_duration1: i32,
    rain_duration2: i32,
    rain_duration3: i32,
    rain_duration4: i32,
    snow_chance1: i32,
    snow_chance2: i32,
    snow_chance3: i32,
    snow_chance4: i32,
    snow_duration1: i32,
    snow_duration2: i32,
    snow_duration3: i32,
    snow_duration4: i32,
    underworld: f32,
    max_z: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct ObjectRow {
    id: i32,
    xpos: f32,
    ypos: f32,
    zpos: f32,
    heading: f32,
    objectname: String,
    #[sqlx(rename = "type")]
    object_type: i32,
    size: f32,
    incline: i32,
    tilt_x: f32,
    tilt_y: f32,
}

#[derive(Debug, sqlx::FromRow)]
struct DoorRow {
    name: String,
    pos_y: f32,
    pos_x: f32,
    pos_z: f32,
    heading: f32,
    incline: i32,
    size: i16,
    doorid: i16,
    opentype: i16,
    invert_state: i32,
    door_param: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct ZonePointRow {
    number: i32,
    target_y: f32,
    target_x: f32,
    target_z: f32,
    target_heading: f32,
    target_zone_id: i32,
    target_instance: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct ZoneSpawnRow {
    npc_name: String,
    lastname: Option<String>,
    level: i16,
    race: i16,
    class: i16,
    gender: i16,
    bodytype: i32,
    hp: i64,
    size: f32,
    runspeed: f32,
    walkspeed: f32,
    texture: i16,
    helmtexture: i16,
    light: i16,
    findable: i16,
    flymode: i16,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
}

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
    char_zone_short: String,
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
            char_zone_short: String::new(),
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
                let (encode_key, compress) = match phase {
                    ConnectionPhase::Zone => (0xFFFFFFFF_u32, true),
                    _ => (0_u32, false),
                };
                let session = EqSession::new(addr, connect_code, max_packet_size, crc_bytes, encode_key, compress);
                let response = packet::build_session_response(
                    connect_code,
                    session.encode_key,
                    session.crc_bytes,
                    session.max_packet_size,
                    if compress { 1 } else { 0 },
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

    // Non-zero first byte = raw app packet (no protocol framing)
    if raw[0] != 0x00 {
        // CRC strip
        if session.crc_bytes > 0 {
            if !eq_protocol::codec::verify_and_strip_crc(&mut raw_owned, session.encode_key, session.crc_bytes) {
                return Ok(());
            }
        }
        // Decompress starting at byte 1 (byte 0 is part of app data)
        if session.compress && raw_owned.len() > 1 {
            match eq_protocol::codec::decompress(&raw_owned[1..]) {
                Ok(decompressed) => {
                    let first = raw_owned[0];
                    raw_owned.clear();
                    raw_owned.push(first);
                    raw_owned.extend_from_slice(&decompressed);
                }
                Err(_) => return Ok(()),
            }
        }
        // Treat as raw app packet: first 2 bytes = opcode (LE)
        if raw_owned.len() >= 2 {
            let app_opcode = u16::from_le_bytes([raw_owned[0], raw_owned[1]]);
            let app_data = &raw_owned[2..];
            dispatch_app_packet(session, &mut state.client_states, addr, socket, phase, app_opcode, app_data, world_state).await?;
        }
        return Ok(());
    }

    // CRC decode: only SessionRequest/SessionResponse/OutOfSession are exempt (per EQEmu PacketCanBeEncoded)
    let proto_op = raw[1];
    match proto_op {
        eq_protocol::OP_SESSION_REQUEST | eq_protocol::OP_SESSION_RESPONSE | eq_protocol::OP_OUT_OF_SESSION => {}
        _ => {
            if !session.decode_packet(&mut raw_owned) {
                return Ok(());
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
    if opcode == opcodes::OP_APP_COMBINED {
        let mut offset = 0;
        while offset < data.len() {
            let sub_len = data[offset] as usize;
            offset += 1;
            if offset + sub_len > data.len() || sub_len < 2 {
                break;
            }
            let sub_opcode = u16::from_le_bytes([data[offset], data[offset + 1]]);
            let sub_data = &data[offset + 2..offset + sub_len];
            Box::pin(dispatch_app_packet(
                session, client_states, addr, socket, phase, sub_opcode, sub_data, world_state,
            ))
            .await?;
            offset += sub_len;
        }
        return Ok(());
    }

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
    let mut buf = if session.compress && data.len() > 2 {
        let mut b = Vec::from(&data[..2]);
        b.extend_from_slice(&eq_protocol::codec::compress(&data[2..]));
        b
    } else {
        data.to_vec()
    };
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
    let mut app_payload = Vec::with_capacity(2 + app_data.len());
    app_payload.extend_from_slice(&app_opcode.to_le_bytes());
    app_payload.extend_from_slice(app_data);

    let proto_header = 2usize; // [0x00][opcode]
    let seq_size = 2usize;
    let compress_flag = if session.compress { 1usize } else { 0 };
    let crc_size = session.crc_bytes as usize;
    let max_pkt = session.max_packet_size as usize;
    let single_size = proto_header + compress_flag + seq_size + app_payload.len() + crc_size;

    if single_size <= max_pkt {
        let pkt = session.build_app_packet(app_opcode, app_data);
        debug!(opcode = format!("0x{app_opcode:04X}"), app_bytes = app_data.len(), udp_bytes = pkt.len(), "TX");
        socket.send_to(&pkt, addr).await?;
    } else {
        let total_size = app_payload.len() as u32;
        let per_frag_overhead = proto_header + compress_flag + seq_size + crc_size;
        let first_data_cap = max_pkt - per_frag_overhead - 4; // 4 for total_size
        let subsequent_data_cap = max_pkt - per_frag_overhead;

        let mut offset = 0;
        let mut frag_count = 0u32;
        while offset < app_payload.len() {
            let (cap, is_first) = if offset == 0 {
                (first_data_cap, true)
            } else {
                (subsequent_data_cap, false)
            };
            let end = (offset + cap).min(app_payload.len());
            let chunk = &app_payload[offset..end];

            let seq = session.next_sequence_out();
            let mut frag_data = Vec::new();
            frag_data.extend_from_slice(&seq.to_be_bytes());
            if is_first {
                frag_data.extend_from_slice(&total_size.to_be_bytes());
            }
            frag_data.extend_from_slice(chunk);

            let encoded = if session.compress {
                eq_protocol::codec::compress(&frag_data)
            } else {
                frag_data
            };

            let mut buf = Vec::with_capacity(max_pkt);
            buf.push(0x00);
            buf.push(eq_protocol::OP_FRAGMENT);
            buf.extend_from_slice(&encoded);
            eq_protocol::codec::append_crc(&mut buf, session.encode_key, session.crc_bytes);

            socket.send_to(&buf, addr).await?;
            offset = end;
            frag_count += 1;
        }
        debug!(
            opcode = format!("0x{app_opcode:04X}"),
            total_bytes = total_size,
            fragments = frag_count,
            "TX fragmented"
        );
    }
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
                    entity_id: cs.player_spawn_id,
                }
            } else {
                warn!(character = %cs.char_name, "Zone: character not found in DB, using defaults");
                PlayerProfileData {
                    name: cs.char_name.clone(), last_name: String::new(),
                    race: 9, class_id: 1, level: 1, gender: 0, deity: 0,
                    x: -99.0, y: -585.0, z: 27.0, heading: 0.0,
                    zone_id: 52, face: 0, hair_color: 0, beard_color: 0,
                    eye_color_1: 0, eye_color_2: 0, hair_style: 0, beard: 0,
                    entity_id: cs.player_spawn_id,
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

            let mut pp = structs::build_player_profile_full(&ppd);

            // Load skills from DB and write into PP at offset 4460 (100 x u32)
            if let Some(ref r) = record {
                let skills: Vec<(i16, i16)> = sqlx::query_as(
                    "SELECT skill_id, value FROM character_skills WHERE id = $1"
                )
                .bind(r.id)
                .fetch_all(&world_state.pool)
                .await?;
                for (skill_id, value) in &skills {
                    let idx = *skill_id as usize;
                    if idx < 100 {
                        let off = 4460 + idx * 4;
                        pp[off..off + 4].copy_from_slice(&(*value as u32).to_le_bytes());
                    }
                }
                structs::recompute_pp_checksum(&mut pp);
                info!(count = skills.len(), "Zone: loaded skills from DB into PlayerProfile");
            }
            info!(
                "PP dump: checksum={:08X} gender={} race={} class={} level={} zone_id_at_13276={} name_at_12940={}",
                u32::from_le_bytes([pp[0], pp[1], pp[2], pp[3]]),
                u32::from_le_bytes([pp[4], pp[5], pp[6], pp[7]]),
                u32::from_le_bytes([pp[8], pp[9], pp[10], pp[11]]),
                u32::from_le_bytes([pp[12], pp[13], pp[14], pp[15]]),
                pp[20],
                u16::from_le_bytes([pp[13276], pp[13277]]),
                String::from_utf8_lossy(&pp[12940..12940+20]).trim_end_matches('\0'),
            );
            info!(
                "PP bytes[0..40]: {}",
                pp[..40].iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ")
            );
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
                x, y, z: z + 6.0, heading, size,
                npc_type: 0, cur_hp: 1, max_hp: 100, body_type: 0,
                run_speed: 0.7, walk_speed: 0.46,
                findable: 1, light: 0, texture: 0, helm_texture: 0,
                guild_id: 0xFFFFFFFF,
            });
            send_app_packet(session, socket, addr, opcodes::OP_ZONE_ENTRY, &player_spawn).await?;

            let zone_short = if cs.char_zone_short.is_empty() { "innothule".to_string() } else { cs.char_zone_short.clone() };
            let spawns = sqlx::query_as::<_, ZoneSpawnRow>(
                "SELECT n.name AS npc_name, n.lastname, n.level, n.race, n.class, \
                 n.gender, n.bodytype, n.hp, n.size, n.runspeed, n.walkspeed, \
                 n.texture, n.helmtexture, n.light, n.findable, n.flymode, \
                 s.x, s.y, s.z, s.heading \
                 FROM spawn2 s \
                 JOIN spawnentry se ON s.spawngroupid = se.spawngroupid \
                 JOIN npc_types n ON se.npcid = n.id \
                 WHERE s.zone = $1 AND (s.version = 0 OR s.version = -1)"
            )
            .bind(&zone_short)
            .fetch_all(&world_state.pool)
            .await?;

            let mut bulk_spawns = Vec::new();
            for row in &spawns {
                let npc_id = cs.alloc_spawn_id();
                let spawn = structs::build_spawn_struct(&SpawnData {
                    spawn_id: npc_id,
                    name: row.npc_name.replace('#', ""),
                    last_name: row.lastname.clone().unwrap_or_default(),
                    level: row.level as u8,
                    race: row.race as u32,
                    class_id: row.class as u8,
                    gender: row.gender as u8,
                    deity: 0,
                    x: row.x, y: row.y, z: row.z, heading: row.heading,
                    size: if row.size > 0.0 { row.size } else { 6.0 },
                    npc_type: 1,
                    cur_hp: 100,
                    max_hp: 100,
                    body_type: row.bodytype as u8,
                    run_speed: if row.runspeed > 0.0 { row.runspeed } else { 0.7 },
                    walk_speed: if row.walkspeed > 0.0 { row.walkspeed } else { 0.46 },
                    findable: row.findable as u8,
                    light: row.light as u8,
                    texture: row.texture as u8,
                    helm_texture: row.helmtexture as u8,
                    guild_id: 0,
                });
                bulk_spawns.extend_from_slice(&spawn);
            }
            if !bulk_spawns.is_empty() {
                send_app_packet(session, socket, addr, opcodes::OP_ZONE_SPAWNS, &bulk_spawns).await?;
            }
            info!(count = spawns.len(), zone = %zone_short, "Zone: sent bulk NPC spawns via OP_ZoneSpawns");

            send_app_packet(session, socket, addr, opcodes::OP_CHAR_INVENTORY, &0u32.to_le_bytes()).await?;
            send_app_packet(session, socket, addr, opcodes::OP_TIME_OF_DAY, &structs::build_time_of_day(14, 0, 1, 3100)).await?;
            send_app_packet(session, socket, addr, opcodes::OP_WEATHER, &structs::build_weather(0, 0)).await?;
        }

        opcodes::OP_REQ_NEW_ZONE => {
            let zone_id = cs.char_zone_id.unwrap_or(46);
            let zr = sqlx::query_as::<_, ZoneDbRow>(
                "SELECT short_name, long_name, safe_x, safe_y, safe_z, \
                 minclip, maxclip, fog_minclip, fog_maxclip, \
                 fog_minclip2, fog_maxclip2, fog_minclip3, fog_maxclip3, \
                 fog_minclip4, fog_maxclip4, \
                 fog_red, fog_green, fog_blue, \
                 fog_red2, fog_green2, fog_blue2, \
                 fog_red3, fog_green3, fog_blue3, \
                 fog_red4, fog_green4, fog_blue4, \
                 fog_density, sky, ztype, zone_exp_multiplier::float4 AS zone_exp_multiplier, gravity, time_type, \
                 rain_chance1, rain_chance2, rain_chance3, rain_chance4, \
                 rain_duration1, rain_duration2, rain_duration3, rain_duration4, \
                 snow_chance1, snow_chance2, snow_chance3, snow_chance4, \
                 snow_duration1, snow_duration2, snow_duration3, snow_duration4, \
                 underworld, max_z \
                 FROM zone WHERE zoneidnumber = $1"
            )
            .bind(zone_id)
            .fetch_one(&world_state.pool)
            .await?;

            let zd = ZoneData {
                short_name: zr.short_name, long_name: zr.long_name,
                zone_id: zone_id as u16,
                safe_x: zr.safe_x, safe_y: zr.safe_y, safe_z: zr.safe_z,
                minclip: zr.minclip, maxclip: zr.maxclip,
                fog_minclip: [zr.fog_minclip, zr.fog_minclip2, zr.fog_minclip3, zr.fog_minclip4],
                fog_maxclip: [zr.fog_maxclip, zr.fog_maxclip2, zr.fog_maxclip3, zr.fog_maxclip4],
                fog_red: [zr.fog_red as u8, zr.fog_red2 as u8, zr.fog_red3 as u8, zr.fog_red4 as u8],
                fog_green: [zr.fog_green as u8, zr.fog_green2 as u8, zr.fog_green3 as u8, zr.fog_green4 as u8],
                fog_blue: [zr.fog_blue as u8, zr.fog_blue2 as u8, zr.fog_blue3 as u8, zr.fog_blue4 as u8],
                fog_density: zr.fog_density,
                sky: zr.sky as u8, ztype: zr.ztype as u8,
                zone_exp_multiplier: zr.zone_exp_multiplier,
                gravity: zr.gravity, time_type: zr.time_type as u8,
                rain_chance: [zr.rain_chance1 as u8, zr.rain_chance2 as u8, zr.rain_chance3 as u8, zr.rain_chance4 as u8],
                rain_duration: [zr.rain_duration1 as u8, zr.rain_duration2 as u8, zr.rain_duration3 as u8, zr.rain_duration4 as u8],
                snow_chance: [zr.snow_chance1 as u8, zr.snow_chance2 as u8, zr.snow_chance3 as u8, zr.snow_chance4 as u8],
                snow_duration: [zr.snow_duration1 as u8, zr.snow_duration2 as u8, zr.snow_duration3 as u8, zr.snow_duration4 as u8],
                underworld: zr.underworld, max_z: zr.max_z as f32,
            };
            info!(zone = %zd.short_name, "Zone: sending zone config from DB");
            let nz = structs::build_new_zone_struct(&cs.char_name, &zd);
            send_app_packet(session, socket, addr, opcodes::OP_NEW_ZONE, &nz).await?;
        }

        opcodes::OP_REQ_CLIENT_SPAWN => {
            info!("Zone: sending zone contents and ready signals");
            let zone_short = if cs.char_zone_short.is_empty() { "innothule".to_string() } else { cs.char_zone_short.clone() };

            let door_rows = sqlx::query_as::<_, DoorRow>(
                "SELECT name, pos_y, pos_x, pos_z, heading, incline, size, \
                 doorid, opentype, invert_state, door_param \
                 FROM doors WHERE zone = $1"
            )
            .bind(&zone_short)
            .fetch_all(&world_state.pool)
            .await?;

            if door_rows.is_empty() {
                send_app_packet(session, socket, addr, opcodes::OP_SPAWN_DOOR, &0u32.to_le_bytes()).await?;
            } else {
                let door_struct_size = 80usize;
                let mut door_buf = vec![0u8; door_rows.len() * door_struct_size];
                for (i, dr) in door_rows.iter().enumerate() {
                    let off = i * door_struct_size;
                    let name_bytes = dr.name.as_bytes();
                    let name_len = name_bytes.len().min(31);
                    door_buf[off..off + name_len].copy_from_slice(&name_bytes[..name_len]);
                    door_buf[off + 32..off + 36].copy_from_slice(&dr.pos_y.to_le_bytes());
                    door_buf[off + 36..off + 40].copy_from_slice(&dr.pos_x.to_le_bytes());
                    door_buf[off + 40..off + 44].copy_from_slice(&dr.pos_z.to_le_bytes());
                    door_buf[off + 44..off + 48].copy_from_slice(&dr.heading.to_le_bytes());
                    door_buf[off + 48..off + 52].copy_from_slice(&(dr.incline as u32).to_le_bytes());
                    door_buf[off + 52..off + 54].copy_from_slice(&(dr.size as u16).to_le_bytes());
                    door_buf[off + 60] = dr.doorid as u8;
                    door_buf[off + 61] = dr.opentype as u8;
                    door_buf[off + 63] = dr.invert_state as u8;
                    door_buf[off + 64..off + 68].copy_from_slice(&(dr.door_param as u32).to_le_bytes());
                    door_buf[off + 77] = 0x01;
                    door_buf[off + 79] = 0x01;
                }
                send_app_packet(session, socket, addr, opcodes::OP_SPAWN_DOOR, &door_buf).await?;
                info!(count = door_rows.len(), zone = %zone_short, "Zone: sent doors from DB");
            }

            // Ground objects (OP_GroundSpawn per object)
            let zone_id = cs.char_zone_id.unwrap_or(46);
            let obj_rows = sqlx::query_as::<_, ObjectRow>(
                "SELECT id, xpos, ypos, zpos, heading, objectname, type, size, incline, tilt_x, tilt_y \
                 FROM object WHERE zoneid = $1"
            )
            .bind(zone_id)
            .fetch_all(&world_state.pool)
            .await?;
            for (i, obj) in obj_rows.iter().enumerate() {
                let mut obuf = vec![0u8; 96];
                obuf[8..12].copy_from_slice(&obj.size.to_le_bytes());
                obuf[12..16].copy_from_slice(&((i as u32) + 1).to_le_bytes()); // drop_id
                obuf[16..18].copy_from_slice(&(zone_id as u16).to_le_bytes());
                obuf[20..24].copy_from_slice(&(obj.incline as u32).to_le_bytes());
                obuf[28..32].copy_from_slice(&obj.tilt_x.to_le_bytes());
                obuf[32..36].copy_from_slice(&obj.tilt_y.to_le_bytes());
                obuf[36..40].copy_from_slice(&obj.heading.to_le_bytes());
                obuf[40..44].copy_from_slice(&obj.zpos.to_le_bytes());
                obuf[44..48].copy_from_slice(&obj.xpos.to_le_bytes());
                obuf[48..52].copy_from_slice(&obj.ypos.to_le_bytes());
                let name_bytes = obj.objectname.as_bytes();
                let name_len = name_bytes.len().min(31);
                obuf[52..52 + name_len].copy_from_slice(&name_bytes[..name_len]);
                obuf[88..92].copy_from_slice(&(obj.object_type as u32).to_le_bytes());
                obuf[92..96].copy_from_slice(&0xFFu32.to_le_bytes());
                send_app_packet(session, socket, addr, opcodes::OP_GROUND_SPAWN, &obuf).await?;
            }
            if !obj_rows.is_empty() {
                info!(count = obj_rows.len(), "Zone: sent ground objects from DB");
            }

            let zone_short = if cs.char_zone_short.is_empty() { "innothule".to_string() } else { cs.char_zone_short.clone() };
            let zp_rows = sqlx::query_as::<_, ZonePointRow>(
                "SELECT number, target_y, target_x, target_z, target_heading, \
                 target_zone_id, target_instance \
                 FROM zone_points WHERE zone = $1"
            )
            .bind(&zone_short)
            .fetch_all(&world_state.pool)
            .await?;

            let count = zp_rows.len() as u32;
            let entry_size = 24usize; // ZonePoint_Entry: u32 + f32 + f32 + f32 + f32 + u16 + u16
            let mut zp_buf = Vec::with_capacity(4 + (count as usize + 1) * entry_size);
            zp_buf.extend_from_slice(&count.to_le_bytes());
            for row in &zp_rows {
                zp_buf.extend_from_slice(&(row.number as u32).to_le_bytes());
                zp_buf.extend_from_slice(&row.target_y.to_le_bytes());
                zp_buf.extend_from_slice(&row.target_x.to_le_bytes());
                zp_buf.extend_from_slice(&row.target_z.to_le_bytes());
                zp_buf.extend_from_slice(&row.target_heading.to_le_bytes());
                zp_buf.extend_from_slice(&(row.target_zone_id as u16).to_le_bytes());
                zp_buf.extend_from_slice(&(row.target_instance as u16).to_le_bytes());
            }
            zp_buf.extend_from_slice(&[0u8; 24]); // extra empty entry per EQEmu
            send_app_packet(session, socket, addr, opcodes::OP_SEND_ZONE_POINTS, &zp_buf).await?;
            info!(count, zone = %zone_short, "Zone: sent zone points from DB");

            send_app_packet(session, socket, addr, opcodes::OP_SEND_AA_STATS, &[]).await?;
            send_app_packet(session, socket, addr, opcodes::OP_SEND_EXP_ZONEIN, &[]).await?;
            send_app_packet(session, socket, addr, opcodes::OP_WORLD_OBJECTS_SENT, &[]).await?;
            info!("Zone: sent zone ready signals");
        }

        opcodes::OP_CLIENT_READY => {
            let sa = structs::build_spawn_appearance(cs.player_spawn_id, 0x10, cs.player_spawn_id);
            send_app_packet(session, socket, addr, opcodes::OP_SPAWN_APPEARANCE, &sa).await?;

            // OP_ExpUpdate: exp ratio (0-330) + aa exp ratio
            let mut exp_buf = [0u8; 8];
            exp_buf[0..4].copy_from_slice(&0u32.to_le_bytes()); // exp = 0 (bottom of level)
            exp_buf[4..8].copy_from_slice(&0u32.to_le_bytes()); // aaxp = 0
            send_app_packet(session, socket, addr, opcodes::OP_EXP_UPDATE, &exp_buf).await?;

            // OP_RaidUpdate: ZoneInSendName_Struct (136 bytes)
            let mut raid_buf = vec![0u8; 136];
            raid_buf[0..4].copy_from_slice(&0x0Au32.to_le_bytes()); // unknown0 = 0x0A
            let name_bytes = cs.char_name.as_bytes();
            let name_len = name_bytes.len().min(63);
            raid_buf[4..4 + name_len].copy_from_slice(&name_bytes[..name_len]);
            raid_buf[68..68 + name_len].copy_from_slice(&name_bytes[..name_len]);
            send_app_packet(session, socket, addr, opcodes::OP_RAID_UPDATE, &raid_buf).await?;

            info!(character = %cs.char_name, "=== CLIENT IN ZONE ===");
        }

        opcodes::OP_CLIENT_UPDATE => {}
        opcodes::OP_ACK_PACKET => {}
        opcodes::OP_SEND_AA_TABLE => {
            send_app_packet(session, socket, addr, opcodes::OP_SEND_AA_TABLE, &[]).await?;
        }
        opcodes::OP_UPDATE_AA => {
            send_app_packet(session, socket, addr, opcodes::OP_UPDATE_AA, &[]).await?;
        }
        opcodes::OP_SEND_TRIBUTES => {
            send_app_packet(session, socket, addr, opcodes::OP_SEND_TRIBUTES, &[]).await?;
        }
        opcodes::OP_GUILD_TRIBUTES => {
            send_app_packet(session, socket, addr, opcodes::OP_GUILD_TRIBUTES, &[]).await?;
        }
        opcodes::OP_SEND_EXP_ZONEIN => {
            send_app_packet(session, socket, addr, opcodes::OP_SEND_EXP_ZONEIN, &[]).await?;
        }
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
