use adif_proto::adif::{self, packet::Payload, Packet};
use bevy_ecs::prelude::*;

use crate::ecs::components::*;

const VISIBILITY_RANGE: f32 = 600.0;

pub fn distance_2d(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    (dx * dx + dy * dy).sqrt()
}

pub fn build_position_broadcast(
    identity: &Identity,
    position: &Position,
    velocity: &Velocity,
    npc_template: Option<&NpcTemplate>,
) -> Packet {
    Packet {
        sequence: 0,
        timestamp: 0,
        payload: Some(Payload::PositionUpdate(adif::PositionUpdate {
            entity_id: identity.entity_id,
            position: Some(adif::Vec3 {
                x: position.x,
                y: position.y,
                z: position.z,
            }),
            velocity: Some(adif::Vec3 {
                x: velocity.x,
                y: velocity.y,
                z: velocity.z,
            }),
            heading: position.heading,
            heading_delta: velocity.heading_delta,
            animation: npc_template.map_or(0, |t| t.animation),
        })),
    }
}

pub fn in_range(x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
    distance_2d(x1, y1, x2, y2) <= VISIBILITY_RANGE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_calculation() {
        assert!((distance_2d(0.0, 0.0, 3.0, 4.0) - 5.0).abs() < 0.001);
        assert!((distance_2d(10.0, 10.0, 10.0, 10.0)).abs() < 0.001);
    }

    #[test]
    fn range_check() {
        assert!(in_range(0.0, 0.0, 100.0, 100.0));
        assert!(!in_range(0.0, 0.0, 500.0, 500.0));
    }

    #[test]
    fn builds_position_packet() {
        let id = Identity {
            entity_id: 42,
            kind: EntityKind::Npc,
            name: "test".to_string(),
            last_name: String::new(),
            race: 1, class_id: 1, level: 1, gender: 0, deity: 0,
        };
        let pos = Position { x: 10.0, y: 20.0, z: 5.0, heading: 90.0 };
        let vel = Velocity { x: 1.0, y: 0.0, z: 0.0, heading_delta: 0.0 };

        let packet = build_position_broadcast(&id, &pos, &vel, None);
        match packet.payload {
            Some(Payload::PositionUpdate(u)) => {
                assert_eq!(u.entity_id, 42);
                let p = u.position.unwrap();
                assert!((p.x - 10.0).abs() < f32::EPSILON);
            }
            _ => panic!("Expected PositionUpdate"),
        }
    }
}
