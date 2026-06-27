use std::sync::Arc;

use bevy_ecs::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use adif_proto::adif::{self, packet::Payload, Packet};

use crate::ecs::components::*;
use crate::ecs::spawn_convert::build_spawn_from_world;
use crate::movement::{self, PositionChanged};
use super::framing::{read_packet, write_packet};
use super::session::{OutboundQueue, SessionManager, SessionState};

pub type SharedSessions = Arc<Mutex<SessionManager>>;
pub type SharedWorld = Arc<Mutex<World>>;

pub async fn start_listener(
    port: u16,
    sessions: SharedSessions,
    world: SharedWorld,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    info!(port, "TCP listener started");

    loop {
        let (stream, addr) = listener.accept().await?;
        info!(addr = %addr, "Client connected");

        let sessions = Arc::clone(&sessions);
        let world = Arc::clone(&world);

        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, addr, sessions, world).await {
                warn!(addr = %addr, error = %e, "Client handler error");
            }
        });
    }
}

async fn handle_client(
    mut stream: TcpStream,
    addr: std::net::SocketAddr,
    sessions: SharedSessions,
    world: SharedWorld,
) -> anyhow::Result<()> {
    let (session_id, outbound) = sessions.lock().await.create_session(addr);

    let result = client_loop(&mut stream, session_id, &outbound, &sessions, &world).await;

    sessions.lock().await.remove_session(session_id);
    info!(session = session_id, "Client disconnected");

    result
}

async fn client_loop(
    stream: &mut TcpStream,
    session_id: u32,
    outbound: &OutboundQueue,
    sessions: &SharedSessions,
    world: &SharedWorld,
) -> anyhow::Result<()> {
    let mut seq: u32 = 0;

    loop {
        // Drain outbound queue
        {
            let mut queue = outbound.lock().await;
            for packet in queue.drain(..) {
                write_packet(stream, &packet).await?;
            }
        }

        // Try to read a packet (with timeout for responsiveness)
        let packet = tokio::select! {
            result = read_packet(stream) => {
                match result {
                    Ok(Some(p)) => p,
                    Ok(None) => return Ok(()), // EOF
                    Err(e) => return Err(e),
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
                continue; // No data ready, loop back to drain outbound
            }
        };

        let state = sessions.lock().await.get(session_id)
            .map(|s| s.state)
            .unwrap_or(SessionState::Connected);

        match packet.payload {
            Some(Payload::SessionRequest(req)) => {
                info!(session = session_id, version = req.protocol_version, "Session request");
                seq += 1;
                let response = Packet {
                    sequence: seq,
                    timestamp: 0,
                    payload: Some(Payload::SessionResponse(adif::SessionResponse {
                        accepted: true,
                        session_id,
                        reject_reason: String::new(),
                    })),
                };
                write_packet(stream, &response).await?;
            }

            Some(Payload::AuthRequest(req)) => {
                info!(session = session_id, account = %req.account_name, "Auth request");
                // Accept all auth for now — real auth comes with login server integration
                sessions.lock().await.authenticate(session_id, 1, req.account_name.clone());
                seq += 1;
                let response = Packet {
                    sequence: seq,
                    timestamp: 0,
                    payload: Some(Payload::AuthResponse(adif::AuthResponse {
                        success: true,
                        account_id: 1,
                        reject_reason: String::new(),
                    })),
                };
                write_packet(stream, &response).await?;
            }

            Some(Payload::ZoneEntryRequest(_)) if state == SessionState::Authenticated => {
                info!(session = session_id, "Zone entry request — sending spawns");
                sessions.lock().await.enter_zone(session_id, 0);

                // Collect all spawns while holding the lock, then release before writing
                let spawn_packets: Vec<Packet> = {
                    let mut w = world.lock().await;
                    let mut packets = Vec::new();
                    let mut query = w.query::<Entity>();
                    for entity in query.iter(&w) {
                        if let Some(spawn) = build_spawn_from_world(&w, entity) {
                            seq += 1;
                            packets.push(Packet {
                                sequence: seq,
                                timestamp: 0,
                                payload: Some(Payload::Spawn(spawn)),
                            });
                        }
                    }
                    packets
                };

                let spawn_count = spawn_packets.len();
                for packet in &spawn_packets {
                    write_packet(stream, packet).await?;
                }

                seq += 1;
                let zone_ready = Packet {
                    sequence: seq,
                    timestamp: 0,
                    payload: Some(Payload::ZoneReady(adif::ZoneReady {
                        zone_id: 0,
                    })),
                };
                write_packet(stream, &zone_ready).await?;

                info!(session = session_id, spawns = spawn_count, "Zone entry complete");
            }

            Some(Payload::PositionUpdate(update)) if state == SessionState::InZone => {
                let entity_id = update.entity_id;
                let mut w = world.lock().await;

                // Find the entity by entity_id and update its position
                let mut found = false;
                let mut query = w.query::<(&Identity, &mut Position, &mut Velocity, &MovementSpeed)>();
                for (id, mut pos, mut vel, speed) in query.iter_mut(&mut w) {
                    if id.entity_id == entity_id {
                        let proposed_pos = update.position.as_ref();
                        let proposed_vel = update.velocity.as_ref();
                        let (px, py, pz) = proposed_pos.map_or((0.0, 0.0, 0.0), |p| (p.x, p.y, p.z));

                        match movement::validate_position_update(&pos, px, py, pz, speed, 0.032) {
                            movement::ValidationResult::Valid => {
                                movement::apply_position_update(
                                    &mut pos, &mut vel,
                                    px, py, pz, update.heading,
                                    proposed_vel.map_or(0.0, |v| v.x),
                                    proposed_vel.map_or(0.0, |v| v.y),
                                    proposed_vel.map_or(0.0, |v| v.z),
                                    update.heading_delta,
                                );
                            }
                            movement::ValidationResult::SpeedHack { distance, max_allowed } => {
                                warn!(
                                    session = session_id,
                                    entity = entity_id,
                                    distance,
                                    max_allowed,
                                    "Speed hack detected — ignoring update"
                                );
                            }
                            movement::ValidationResult::OutOfBounds { .. } => {
                                warn!(session = session_id, "Out of bounds — ignoring");
                            }
                        }
                        found = true;
                        break;
                    }
                }

                if found {
                    // Mark for broadcast (can't do this inside the query loop)
                    let mut query2 = w.query::<&Identity>();
                    for id in query2.iter(&w) {
                        if id.entity_id == entity_id {
                            // We'd mark PositionChanged here but can't get Entity from Identity alone
                            // in this architecture. Broadcasting will be tick-based in the game loop.
                            break;
                        }
                    }
                }
            }

            Some(Payload::Heartbeat(_)) => {
                // Client is alive, no response needed
            }

            Some(Payload::Disconnect(d)) => {
                info!(session = session_id, reason = ?d.reason, "Client disconnect");
                return Ok(());
            }

            Some(other) => {
                warn!(session = session_id, payload = ?std::mem::discriminant(&other), "Unhandled packet");
            }

            None => {}
        }
    }
}
