use std::time::{Duration, Instant};

use bevy_ecs::prelude::*;
use tracing::{debug, info, warn};

use crate::ai::systems::{system_ai_aggro_check, system_ai_chase, system_ai_leash};
use crate::combat::systems::{system_auto_attack, system_process_death};
use crate::ecs::components::*;

pub const TICK_RATE_HZ: u32 = 31;
pub const TICK_DURATION: Duration = Duration::from_millis(1000 / TICK_RATE_HZ as u64);

#[derive(Resource)]
pub struct TickState {
    pub tick: u64,
    pub delta: Duration,
    pub elapsed: Duration,
    started_at: Instant,
    last_tick: Instant,
    slow_tick_count: u64,
}

impl TickState {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            tick: 0,
            delta: TICK_DURATION,
            elapsed: Duration::ZERO,
            started_at: now,
            last_tick: now,
            slow_tick_count: 0,
        }
    }

    pub fn advance(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_tick;
        self.elapsed = now - self.started_at;
        self.last_tick = now;
        self.tick += 1;

        if self.delta > TICK_DURATION * 2 {
            self.slow_tick_count += 1;
            if self.slow_tick_count <= 5 || self.slow_tick_count % 100 == 0 {
                warn!(
                    tick = self.tick,
                    delta_ms = self.delta.as_millis(),
                    target_ms = TICK_DURATION.as_millis(),
                    "Slow tick"
                );
            }
        }
    }

    pub fn uptime_secs(&self) -> f64 {
        self.elapsed.as_secs_f64()
    }
}

#[derive(Resource)]
pub struct ZoneTime {
    pub hour: u8,
    pub minute: u8,
    ticks_per_game_minute: u64,
    tick_accumulator: u64,
}

impl ZoneTime {
    pub fn new() -> Self {
        // EQ: 1 real second = 3 game minutes → 1 game hour = 20 real seconds
        // At 31 ticks/sec: ~10 ticks per game minute
        Self {
            hour: 8,
            minute: 0,
            ticks_per_game_minute: (TICK_RATE_HZ / 3).max(1) as u64,
            tick_accumulator: 0,
        }
    }

    pub fn advance(&mut self) {
        self.tick_accumulator += 1;
        if self.tick_accumulator >= self.ticks_per_game_minute {
            self.tick_accumulator = 0;
            self.minute += 1;
            if self.minute >= 60 {
                self.minute = 0;
                self.hour += 1;
                if self.hour >= 24 {
                    self.hour = 0;
                }
            }
        }
    }

    pub fn is_daytime(&self) -> bool {
        self.hour >= 6 && self.hour < 20
    }
}

#[derive(Resource)]
pub struct RespawnQueue {
    entries: Vec<RespawnEntry>,
}

struct RespawnEntry {
    spawn_point_id: i32,
    spawngroupid: i32,
    respawn_at_tick: u64,
}

impl RespawnQueue {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn schedule(&mut self, spawn_point_id: i32, spawngroupid: i32, respawn_time_secs: i32, current_tick: u64) {
        let ticks = (respawn_time_secs as u64) * TICK_RATE_HZ as u64;
        self.entries.push(RespawnEntry {
            spawn_point_id,
            spawngroupid,
            respawn_at_tick: current_tick + ticks,
        });
    }

    pub fn drain_ready(&mut self, current_tick: u64) -> Vec<(i32, i32)> {
        let mut ready = Vec::new();
        self.entries.retain(|e| {
            if current_tick >= e.respawn_at_tick {
                ready.push((e.spawn_point_id, e.spawngroupid));
                false
            } else {
                true
            }
        });
        ready
    }

    pub fn pending_count(&self) -> usize {
        self.entries.len()
    }
}

pub fn system_advance_time(mut zone_time: ResMut<ZoneTime>, tick: Res<TickState>) {
    zone_time.advance();
    if tick.tick % (TICK_RATE_HZ as u64 * 20) == 0 && tick.tick > 0 {
        debug!(
            hour = zone_time.hour,
            minute = zone_time.minute,
            day = if zone_time.is_daytime() { "day" } else { "night" },
            "Zone time"
        );
    }
}

pub fn system_log_heartbeat(tick: Res<TickState>, query: Query<&Identity>) {
    // Log every ~30 seconds
    if tick.tick % (TICK_RATE_HZ as u64 * 30) == 0 && tick.tick > 0 {
        info!(
            tick = tick.tick,
            uptime = format!("{:.0}s", tick.uptime_secs()),
            entities = query.iter().count(),
            "Heartbeat"
        );
    }
}

pub async fn run_loop(world: &mut World, duration: Option<Duration>) {
    let deadline = duration.map(|d| Instant::now() + d);

    world.insert_resource(TickState::new());
    world.insert_resource(ZoneTime::new());
    world.insert_resource(RespawnQueue::new());

    let mut schedule = Schedule::default();
    schedule.add_systems((
        system_advance_time,
        system_ai_aggro_check,
        system_ai_chase,
        system_ai_leash,
        system_auto_attack,
        system_process_death,
        system_log_heartbeat,
    ));

    info!(
        tick_rate = TICK_RATE_HZ,
        tick_ms = TICK_DURATION.as_millis(),
        "Game loop starting"
    );

    loop {
        let tick_start = Instant::now();

        world.resource_mut::<TickState>().advance();
        schedule.run(world);

        if let Some(dl) = deadline {
            if Instant::now() >= dl {
                let tick = world.resource::<TickState>().tick;
                info!(ticks = tick, "Game loop stopped (duration reached)");
                break;
            }
        }

        let tick_elapsed = tick_start.elapsed();
        if tick_elapsed < TICK_DURATION {
            tokio::time::sleep(TICK_DURATION - tick_elapsed).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_state_advances() {
        let mut state = TickState::new();
        assert_eq!(state.tick, 0);
        state.advance();
        assert_eq!(state.tick, 1);
        state.advance();
        assert_eq!(state.tick, 2);
    }

    #[test]
    fn zone_time_wraps_hours() {
        let mut time = ZoneTime::new();
        time.hour = 23;
        time.minute = 59;
        time.tick_accumulator = time.ticks_per_game_minute - 1;
        time.advance();
        assert_eq!(time.hour, 0);
        assert_eq!(time.minute, 0);
    }

    #[test]
    fn zone_time_day_night() {
        let mut time = ZoneTime::new();
        time.hour = 12;
        assert!(time.is_daytime());
        time.hour = 22;
        assert!(!time.is_daytime());
        time.hour = 5;
        assert!(!time.is_daytime());
        time.hour = 6;
        assert!(time.is_daytime());
    }

    #[test]
    fn respawn_queue_drains_ready() {
        let mut queue = RespawnQueue::new();
        queue.schedule(1, 100, 10, 0); // respawn at tick 310
        queue.schedule(2, 200, 5, 0);  // respawn at tick 155

        assert_eq!(queue.pending_count(), 2);

        let ready = queue.drain_ready(100);
        assert!(ready.is_empty());
        assert_eq!(queue.pending_count(), 2);

        let ready = queue.drain_ready(155);
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0], (2, 200));
        assert_eq!(queue.pending_count(), 1);

        let ready = queue.drain_ready(310);
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0], (1, 100));
        assert_eq!(queue.pending_count(), 0);
    }

    #[test]
    fn game_time_rate() {
        // 1 real second = 3 game minutes at 31 Hz
        let mut time = ZoneTime::new();
        time.hour = 0;
        time.minute = 0;
        // Advance for 31 ticks (1 real second)
        for _ in 0..31 {
            time.advance();
        }
        // Should be ~3 game minutes
        assert!(time.minute >= 2 && time.minute <= 4,
            "Expected ~3 game minutes after 31 ticks, got {}", time.minute);
    }
}
