use bevy_ecs::prelude::*;
use tracing::warn;

use crate::ecs::components::*;

const MAX_RUN_SPEED: f32 = 3.0;
const MAX_POSITION_DELTA: f32 = 50.0;

#[derive(Debug)]
pub enum ValidationResult {
    Valid,
    SpeedHack { distance: f32, max_allowed: f32 },
    OutOfBounds { x: f32, y: f32, z: f32 },
}

pub fn validate_position_update(
    current: &Position,
    proposed_x: f32,
    proposed_y: f32,
    proposed_z: f32,
    speed: &MovementSpeed,
    _delta_secs: f32,
) -> ValidationResult {
    let dx = proposed_x - current.x;
    let dy = proposed_y - current.y;
    let dz = proposed_z - current.z;
    let distance = (dx * dx + dy * dy + dz * dz).sqrt();

    let max_allowed = speed.run_speed * MAX_RUN_SPEED * MAX_POSITION_DELTA;
    if distance > max_allowed {
        return ValidationResult::SpeedHack { distance, max_allowed };
    }

    ValidationResult::Valid
}

pub fn apply_position_update(
    position: &mut Position,
    velocity: &mut Velocity,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    vel_x: f32,
    vel_y: f32,
    vel_z: f32,
    heading_delta: f32,
) {
    position.x = x;
    position.y = y;
    position.z = z;
    position.heading = heading;
    velocity.x = vel_x;
    velocity.y = vel_y;
    velocity.z = vel_z;
    velocity.heading_delta = heading_delta;
}

#[derive(Component, Debug)]
pub struct PositionChanged;

pub fn system_clear_position_changed(
    mut commands: Commands,
    query: Query<Entity, With<PositionChanged>>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<PositionChanged>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_movement() {
        let pos = Position { x: 0.0, y: 0.0, z: 0.0, heading: 0.0 };
        let speed = MovementSpeed { run_speed: 0.7, walk_speed: 0.46, fly_mode: 0 };
        let result = validate_position_update(&pos, 5.0, 5.0, 0.0, &speed, 0.032);
        assert!(matches!(result, ValidationResult::Valid));
    }

    #[test]
    fn speed_hack_detected() {
        let pos = Position { x: 0.0, y: 0.0, z: 0.0, heading: 0.0 };
        let speed = MovementSpeed { run_speed: 0.7, walk_speed: 0.46, fly_mode: 0 };
        let result = validate_position_update(&pos, 500.0, 500.0, 0.0, &speed, 0.032);
        assert!(matches!(result, ValidationResult::SpeedHack { .. }));
    }

    #[test]
    fn apply_updates_position() {
        let mut pos = Position { x: 0.0, y: 0.0, z: 0.0, heading: 0.0 };
        let mut vel = Velocity::default();
        apply_position_update(&mut pos, &mut vel, 10.0, 20.0, 5.0, 90.0, 1.0, 0.5, 0.0, 0.0);
        assert!((pos.x - 10.0).abs() < f32::EPSILON);
        assert!((pos.y - 20.0).abs() < f32::EPSILON);
        assert!((pos.heading - 90.0).abs() < f32::EPSILON);
        assert!((vel.x - 1.0).abs() < f32::EPSILON);
    }
}
