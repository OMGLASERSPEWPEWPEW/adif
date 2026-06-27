use adif_proto::adif::{self, packet::Payload, Packet};
use bevy_ecs::prelude::*;
use tracing::info;

use crate::ecs::components::*;
use crate::network::broadcast::distance_2d;

const SAY_RANGE: f32 = 200.0;

pub struct ChatOutput {
    pub packets: Vec<(Option<u32>, Packet)>,
}

impl ChatOutput {
    pub fn new() -> Self {
        Self { packets: Vec::new() }
    }

    fn send_to(&mut self, target_entity_id: Option<u32>, packet: Packet) {
        self.packets.push((target_entity_id, packet));
    }

    fn broadcast(&mut self, packet: Packet) {
        self.packets.push((None, packet));
    }
}

pub fn handle_chat_message(
    msg: &adif::ChatMessage,
    sender_entity_id: u32,
    sender_pos: Option<(f32, f32)>,
    world: &mut World,
) -> ChatOutput {
    let mut output = ChatOutput::new();

    let channel = adif::ChatChannel::try_from(msg.channel)
        .unwrap_or(adif::ChatChannel::Unspecified);

    let response = Packet {
        sequence: 0,
        timestamp: 0,
        payload: Some(Payload::ChatMessage(adif::ChatMessage {
            sender_name: msg.sender_name.clone(),
            target_name: msg.target_name.clone(),
            channel: msg.channel,
            language: msg.language,
            message: msg.message.clone(),
        })),
    };

    match channel {
        adif::ChatChannel::Say => {
            info!(sender = %msg.sender_name, message = %msg.message, "SAY");
            // Send to all players within range
            if let Some((sx, sy)) = sender_pos {
                let mut query = world.query::<(&Identity, &Position)>();
                for (id, pos) in query.iter(world) {
                    if id.entity_id == sender_entity_id {
                        continue;
                    }
                    if id.kind != EntityKind::Player {
                        continue;
                    }
                    if distance_2d(sx, sy, pos.x, pos.y) <= SAY_RANGE {
                        output.send_to(Some(id.entity_id), response.clone());
                    }
                }
            }
            // Echo to sender
            output.send_to(Some(sender_entity_id), response);
        }

        adif::ChatChannel::Shout | adif::ChatChannel::Ooc | adif::ChatChannel::Auction => {
            info!(
                sender = %msg.sender_name,
                channel = ?channel,
                message = %msg.message,
                "ZONE-WIDE"
            );
            output.broadcast(response);
        }

        adif::ChatChannel::Tell => {
            info!(
                sender = %msg.sender_name,
                target = %msg.target_name,
                message = %msg.message,
                "TELL"
            );
            // Find target by name
            let mut query = world.query::<&Identity>();
            let target = query.iter(world).find(|id| {
                id.kind == EntityKind::Player && id.name.eq_ignore_ascii_case(&msg.target_name)
            });

            if let Some(target_id) = target {
                output.send_to(Some(target_id.entity_id), response.clone());
                output.send_to(Some(sender_entity_id), response);
            } else {
                output.send_to(Some(sender_entity_id), Packet {
                    sequence: 0,
                    timestamp: 0,
                    payload: Some(Payload::SystemMessage(adif::SystemMessage {
                        r#type: adif::SystemMessageType::Error as i32,
                        color: 0,
                        text: format!("Player '{}' not found.", msg.target_name),
                    })),
                });
            }
        }

        _ => {
            output.send_to(Some(sender_entity_id), response);
        }
    }

    output
}

pub fn build_who_response(world: &mut World) -> Packet {
    let mut query = world.query::<&Identity>();
    let mut entries = Vec::new();

    for id in query.iter(world) {
        if id.kind != EntityKind::Player {
            continue;
        }
        entries.push(adif::WhoEntry {
            name: id.name.clone(),
            level: id.level,
            race: id.race,
            class_id: id.class_id,
            guild_name: String::new(),
            zone_id: 0,
            anonymous: false,
            lfg: false,
        });
    }

    let total = entries.len() as u32;
    Packet {
        sequence: 0,
        timestamp: 0,
        payload: Some(Payload::WhoResponse(adif::WhoResponse {
            entries,
            total_online: total,
        })),
    }
}

pub fn build_consider(
    entity_id: u32,
    target: &Identity,
    target_health: &Health,
) -> Packet {
    Packet {
        sequence: 0,
        timestamp: 0,
        payload: Some(Payload::Consider(adif::Consider {
            entity_id,
            target_id: target.entity_id,
            faction: 0,
            level: target.level,
            current_hp: target_health.current_hp,
            max_hp: target_health.max_hp,
            pvp: false,
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shout_broadcasts_to_all() {
        let mut world = World::new();
        let msg = adif::ChatMessage {
            sender_name: "Ghouldan".to_string(),
            target_name: String::new(),
            channel: adif::ChatChannel::Shout as i32,
            language: 0,
            message: "Train to zone!".to_string(),
        };

        let output = handle_chat_message(&msg, 1, Some((0.0, 0.0)), &mut world);
        assert_eq!(output.packets.len(), 1);
        assert_eq!(output.packets[0].0, None); // broadcast
    }

    #[test]
    fn tell_to_missing_player_sends_error() {
        let mut world = World::new();
        let msg = adif::ChatMessage {
            sender_name: "Ghouldan".to_string(),
            target_name: "Nobody".to_string(),
            channel: adif::ChatChannel::Tell as i32,
            language: 0,
            message: "hello".to_string(),
        };

        let output = handle_chat_message(&msg, 1, Some((0.0, 0.0)), &mut world);
        assert_eq!(output.packets.len(), 1);
        assert_eq!(output.packets[0].0, Some(1)); // error to sender
        match &output.packets[0].1.payload {
            Some(Payload::SystemMessage(m)) => {
                assert!(m.text.contains("not found"));
            }
            _ => panic!("Expected SystemMessage"),
        }
    }

    #[test]
    fn who_response_filters_npcs() {
        let mut world = World::new();
        world.spawn(Identity {
            entity_id: 1, kind: EntityKind::Player,
            name: "Ghouldan".to_string(), last_name: String::new(),
            race: 5, class_id: 11, level: 50, gender: 0, deity: 0,
        });
        world.spawn(Identity {
            entity_id: 2, kind: EntityKind::Npc,
            name: "an_orc_pawn".to_string(), last_name: String::new(),
            race: 1, class_id: 1, level: 5, gender: 0, deity: 0,
        });

        let packet = build_who_response(&mut world);
        match packet.payload {
            Some(Payload::WhoResponse(r)) => {
                assert_eq!(r.entries.len(), 1);
                assert_eq!(r.entries[0].name, "Ghouldan");
                assert_eq!(r.total_online, 1);
            }
            _ => panic!("Expected WhoResponse"),
        }
    }

    #[test]
    fn consider_builds_packet() {
        let target = Identity {
            entity_id: 42, kind: EntityKind::Npc,
            name: "a_fire_beetle".to_string(), last_name: String::new(),
            race: 11, class_id: 1, level: 3, gender: 0, deity: 0,
        };
        let health = Health {
            current_hp: 25, max_hp: 25,
            current_mana: 0, max_mana: 0,
            current_endurance: 0, max_endurance: 0,
        };

        let packet = build_consider(1, &target, &health);
        match packet.payload {
            Some(Payload::Consider(c)) => {
                assert_eq!(c.entity_id, 1);
                assert_eq!(c.target_id, 42);
                assert_eq!(c.level, 3);
                assert_eq!(c.current_hp, 25);
            }
            _ => panic!("Expected Consider"),
        }
    }
}
