use bevy_ecs::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiState {
    Idle,
    Patrol,
    Aggro,
    Chase,
    Combat,
    Leash,
}

#[derive(Component, Debug)]
pub struct AiBrain {
    pub state: AiState,
    pub home_x: f32,
    pub home_y: f32,
    pub home_z: f32,
    pub aggro_radius: f32,
    pub assist_radius: f32,
    pub leash_radius: f32,
    pub patrol_grid_id: i32,
}

impl AiBrain {
    pub fn new(home_x: f32, home_y: f32, home_z: f32, aggro_radius: f32, assist_radius: f32, patrol_grid_id: i32) -> Self {
        Self {
            state: if patrol_grid_id > 0 { AiState::Patrol } else { AiState::Idle },
            home_x,
            home_y,
            home_z,
            aggro_radius: if aggro_radius > 0.0 { aggro_radius } else { 70.0 },
            assist_radius: if assist_radius > 0.0 { assist_radius } else { 30.0 },
            leash_radius: 600.0,
            patrol_grid_id,
        }
    }

    pub fn distance_from_home(&self, x: f32, y: f32) -> f32 {
        let dx = x - self.home_x;
        let dy = y - self.home_y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn should_leash(&self, x: f32, y: f32) -> bool {
        self.distance_from_home(x, y) > self.leash_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_idle_without_patrol() {
        let brain = AiBrain::new(0.0, 0.0, 0.0, 70.0, 30.0, 0);
        assert_eq!(brain.state, AiState::Idle);
    }

    #[test]
    fn initial_state_patrol_with_grid() {
        let brain = AiBrain::new(0.0, 0.0, 0.0, 70.0, 30.0, 5);
        assert_eq!(brain.state, AiState::Patrol);
    }

    #[test]
    fn leash_detection() {
        let brain = AiBrain::new(100.0, 100.0, 0.0, 70.0, 30.0, 0);
        assert!(!brain.should_leash(200.0, 200.0));
        assert!(brain.should_leash(800.0, 800.0));
    }

    #[test]
    fn default_aggro_radius() {
        let brain = AiBrain::new(0.0, 0.0, 0.0, 0.0, 0.0, 0);
        assert!((brain.aggro_radius - 70.0).abs() < f32::EPSILON);
    }
}
