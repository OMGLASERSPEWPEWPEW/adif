use bevy_ecs::prelude::*;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Waypoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading: f32,
    pub pause_secs: i32,
}

#[derive(Debug, Clone)]
pub struct PatrolGrid {
    pub grid_id: i32,
    pub waypoints: Vec<Waypoint>,
}

#[derive(Component, Debug)]
pub struct PatrolState {
    pub grid_id: i32,
    pub current_waypoint: usize,
    pub pause_remaining_ticks: u64,
    pub moving: bool,
}

impl PatrolState {
    pub fn new(grid_id: i32) -> Self {
        Self {
            grid_id,
            current_waypoint: 0,
            pause_remaining_ticks: 0,
            moving: false,
        }
    }

    pub fn advance(&mut self, waypoint_count: usize) {
        if waypoint_count == 0 {
            return;
        }
        self.current_waypoint = (self.current_waypoint + 1) % waypoint_count;
    }
}

#[derive(sqlx::FromRow)]
struct GridEntryRow {
    gridid: i32,
    number: i32,
    x: f32,
    y: f32,
    z: f32,
    heading: f32,
    pause: i32,
}

pub async fn load_patrol_grids(pool: &PgPool, zone_id: i32) -> anyhow::Result<Vec<PatrolGrid>> {
    let rows = sqlx::query_as::<_, GridEntryRow>(
        "SELECT gridid, number, x, y, z, heading, pause \
         FROM grid_entries \
         WHERE zoneid = $1 \
         ORDER BY gridid, number",
    )
    .bind(zone_id)
    .fetch_all(pool)
    .await?;

    let mut grids: Vec<PatrolGrid> = Vec::new();
    let mut current_id: Option<i32> = None;

    for row in &rows {
        if current_id != Some(row.gridid) {
            grids.push(PatrolGrid {
                grid_id: row.gridid,
                waypoints: Vec::new(),
            });
            current_id = Some(row.gridid);
        }
        if let Some(grid) = grids.last_mut() {
            grid.waypoints.push(Waypoint {
                x: row.x,
                y: row.y,
                z: row.z,
                heading: row.heading,
                pause_secs: row.pause,
            });
        }
    }

    Ok(grids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patrol_state_advances() {
        let mut state = PatrolState::new(1);
        assert_eq!(state.current_waypoint, 0);
        state.advance(5);
        assert_eq!(state.current_waypoint, 1);
        state.advance(5);
        assert_eq!(state.current_waypoint, 2);
    }

    #[test]
    fn patrol_state_wraps() {
        let mut state = PatrolState::new(1);
        state.current_waypoint = 4;
        state.advance(5);
        assert_eq!(state.current_waypoint, 0);
    }

    #[test]
    fn patrol_state_empty_grid() {
        let mut state = PatrolState::new(1);
        state.advance(0);
        assert_eq!(state.current_waypoint, 0);
    }
}
