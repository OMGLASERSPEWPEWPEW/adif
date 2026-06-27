use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct MeleeStats {
    pub min_damage: i32,
    pub max_damage: i32,
    pub attack_delay_ms: u32,
    pub ticks_until_attack: u32,
}

impl MeleeStats {
    pub fn new(min_dmg: i32, max_dmg: i32) -> Self {
        let delay = 2000u32;
        Self {
            min_damage: min_dmg.max(1),
            max_damage: max_dmg.max(min_dmg.max(1)),
            attack_delay_ms: delay,
            ticks_until_attack: 0,
        }
    }

    pub fn attack_interval_ticks(&self) -> u32 {
        (self.attack_delay_ms / 32).max(1)
    }

    pub fn is_ready(&self) -> bool {
        self.ticks_until_attack == 0
    }

    pub fn tick(&mut self) {
        if self.ticks_until_attack > 0 {
            self.ticks_until_attack -= 1;
        }
    }

    pub fn reset_timer(&mut self) {
        self.ticks_until_attack = self.attack_interval_ticks();
    }
}

pub fn calculate_melee_damage(min_dmg: i32, max_dmg: i32, attacker_level: u32, defender_level: u32) -> DamageResult {
    let base = (min_dmg + max_dmg) / 2;

    let level_diff = attacker_level as i32 - defender_level as i32;
    let hit_chance: f32 = (75.0 + level_diff as f32 * 2.0).clamp(5.0, 95.0);

    // Deterministic for now — randomness comes with a proper RNG seed system
    let roll = (attacker_level * 7 + defender_level * 3) % 100;
    if (roll as f32) >= hit_chance {
        return DamageResult::Miss;
    }

    let damage = base.max(1);
    DamageResult::Hit(damage)
}

#[derive(Debug, PartialEq)]
pub enum DamageResult {
    Hit(i32),
    Miss,
}

#[derive(Component, Debug)]
pub struct DeadMarker;

pub fn xp_for_kill(npc_level: u32, player_level: u32) -> u64 {
    let base = npc_level as u64 * npc_level as u64 * 10;
    let diff = npc_level as i64 - player_level as i64;
    let modifier = match diff {
        d if d >= 5 => 200,
        d if d >= 3 => 150,
        d if d >= 0 => 100,
        d if d >= -3 => 75,
        d if d >= -5 => 50,
        d if d >= -10 => 25,
        _ => 5,
    };
    (base * modifier) / 100
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn melee_stats_timer() {
        let mut stats = MeleeStats::new(10, 20);
        assert!(stats.is_ready());
        stats.reset_timer();
        assert!(!stats.is_ready());
        for _ in 0..stats.attack_interval_ticks() {
            stats.tick();
        }
        assert!(stats.is_ready());
    }

    #[test]
    fn melee_stats_clamps_min() {
        let stats = MeleeStats::new(0, 0);
        assert_eq!(stats.min_damage, 1);
        assert_eq!(stats.max_damage, 1);
    }

    #[test]
    fn xp_same_level() {
        let xp = xp_for_kill(10, 10);
        assert_eq!(xp, 1000); // 10*10*10 * 100/100
    }

    #[test]
    fn xp_higher_mob() {
        let xp = xp_for_kill(15, 10);
        assert_eq!(xp, 4500); // 15*15*10 * 200/100
    }

    #[test]
    fn xp_much_lower_mob() {
        let xp = xp_for_kill(5, 50);
        assert_eq!(xp, 12); // 5*5*10 * 5/100 = 12.5 → 12
    }

    #[test]
    fn damage_result() {
        let result = calculate_melee_damage(10, 20, 10, 10);
        // With our deterministic formula, this should be either Hit or Miss
        match result {
            DamageResult::Hit(d) => assert!(d >= 1),
            DamageResult::Miss => {} // acceptable
        }
    }
}
