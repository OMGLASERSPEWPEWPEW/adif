use bevy_ecs::prelude::*;
use sqlx::PgPool;
use tracing::info;

use adif_proto::adif::{self, packet::Payload, Packet};

use crate::ecs::components::*;
use crate::network::broadcast::distance_2d;

#[derive(Debug, Clone)]
pub struct ZonePoint {
    pub id: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading: f32,
    pub target_zone_id: i32,
    pub target_x: f32,
    pub target_y: f32,
    pub target_z: f32,
    pub target_heading: f32,
    pub buffer: f32,
}

impl ZonePoint {
    pub async fn load_for_zone(pool: &PgPool, zone_short_name: &str) -> anyhow::Result<Vec<Self>> {
        let points = sqlx::query_as::<_, ZonePointRow>(
            "SELECT id, x, y, z, heading, target_zone_id, \
             target_x, target_y, target_z, target_heading, \
             COALESCE(buffer, 200.0) as buffer \
             FROM zone_points \
             WHERE zone = $1 AND version = 0",
        )
        .bind(zone_short_name)
        .fetch_all(pool)
        .await?;

        Ok(points
            .into_iter()
            .map(|r| ZonePoint {
                id: r.id,
                x: r.x,
                y: r.y,
                z: r.z,
                heading: r.heading,
                target_zone_id: r.target_zone_id,
                target_x: r.target_x,
                target_y: r.target_y,
                target_z: r.target_z,
                target_heading: r.target_heading,
                buffer: r.buffer,
            })
            .collect())
    }
}

#[derive(sqlx::FromRow)]
struct ZonePointRow {
    id: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    target_zone_id: i32,
    target_x: f32,
    target_y: f32,
    target_z: f32,
    target_heading: f32,
    buffer: f32,
}

#[derive(Resource)]
pub struct ZoneLines {
    points: Vec<ZonePoint>,
}

impl ZoneLines {
    pub fn new(points: Vec<ZonePoint>) -> Self {
        Self { points }
    }

    pub fn check_zone_line(&self, x: f32, y: f32, z: f32) -> Option<&ZonePoint> {
        for point in &self.points {
            let dist = distance_2d(x, y, point.x, point.y);
            if dist <= point.buffer && (z - point.z).abs() < 50.0 {
                return Some(point);
            }
        }
        None
    }

    pub fn point_count(&self) -> usize {
        self.points.len()
    }
}

pub fn build_zone_change_response(zone_point: &ZonePoint) -> Packet {
    Packet {
        sequence: 0,
        timestamp: 0,
        payload: Some(Payload::ZoneChangeResponse(adif::ZoneChangeResponse {
            approved: true,
            target_zone_id: zone_point.target_zone_id as u32,
            target_position: Some(adif::Vec3 {
                x: zone_point.target_x,
                y: zone_point.target_y,
                z: zone_point.target_z,
            }),
            target_heading: zone_point.target_heading,
            deny_reason: String::new(),
        })),
    }
}

#[derive(Component, Debug)]
pub struct CampTimer {
    pub ticks_remaining: u32,
}

impl CampTimer {
    pub fn new() -> Self {
        Self {
            ticks_remaining: 30 * 31, // 30 seconds at 31 Hz
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.ticks_remaining > 0 {
            self.ticks_remaining -= 1;
        }
        self.ticks_remaining == 0
    }

    pub fn seconds_remaining(&self) -> u32 {
        self.ticks_remaining / 31
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zone_line_detection() {
        let lines = ZoneLines::new(vec![
            ZonePoint {
                id: 1,
                x: 100.0, y: 200.0, z: 0.0, heading: 0.0,
                target_zone_id: 2,
                target_x: 50.0, target_y: 50.0, target_z: 0.0, target_heading: 0.0,
                buffer: 30.0,
            },
        ]);

        // Within range
        assert!(lines.check_zone_line(110.0, 200.0, 0.0).is_some());
        // Out of range
        assert!(lines.check_zone_line(500.0, 500.0, 0.0).is_none());
        // Too high
        assert!(lines.check_zone_line(100.0, 200.0, 100.0).is_none());
    }

    #[test]
    fn camp_timer_counts_down() {
        let mut timer = CampTimer::new();
        assert_eq!(timer.seconds_remaining(), 30);
        assert!(!timer.tick());

        for _ in 0..30 * 31 - 2 {
            timer.tick();
        }
        assert_eq!(timer.ticks_remaining, 1);
        assert!(timer.tick());
    }

    #[test]
    fn zone_change_response_packet() {
        let point = ZonePoint {
            id: 1,
            x: 0.0, y: 0.0, z: 0.0, heading: 0.0,
            target_zone_id: 46,
            target_x: -500.0, target_y: 200.0, target_z: 5.0, target_heading: 90.0,
            buffer: 30.0,
        };

        let packet = build_zone_change_response(&point);
        match packet.payload {
            Some(Payload::ZoneChangeResponse(r)) => {
                assert!(r.approved);
                assert_eq!(r.target_zone_id, 46);
                let pos = r.target_position.unwrap();
                assert!((pos.x - (-500.0)).abs() < f32::EPSILON);
            }
            _ => panic!("Expected ZoneChangeResponse"),
        }
    }
}
