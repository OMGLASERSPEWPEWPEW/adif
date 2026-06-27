use bevy_ecs::prelude::*;
use tracing::{debug, info};

use crate::ai::hate_list::HateList;
use crate::ai::state::{AiBrain, AiState};
use crate::ecs::components::*;
use crate::network::broadcast::distance_2d;
use super::damage::{calculate_melee_damage, DeadMarker, MeleeStats};

const MELEE_RANGE: f32 = 15.0;

pub fn system_auto_attack(
    mut attackers: Query<
        (&Identity, &Position, &mut Health, &mut MeleeStats, &HateList),
        (With<AiBrain>, Without<DeadMarker>),
    >,
    mut defenders: Query<
        (&Identity, &Position, &mut Health),
        Without<AiBrain>,
    >,
) {
    for (atk_id, atk_pos, _atk_health, mut melee, hate_list) in attackers.iter_mut() {
        melee.tick();

        if !melee.is_ready() {
            continue;
        }

        let target_id = match hate_list.top_target() {
            Some(id) => id,
            None => continue,
        };

        let target = defenders
            .iter_mut()
            .find(|(id, _, _)| id.entity_id == target_id);

        let (def_id, def_pos, mut def_health) = match target {
            Some(t) => t,
            None => continue,
        };

        let dist = distance_2d(atk_pos.x, atk_pos.y, def_pos.x, def_pos.y);
        if dist > MELEE_RANGE {
            continue;
        }

        let result = calculate_melee_damage(
            melee.min_damage,
            melee.max_damage,
            atk_id.level,
            def_id.level,
        );

        match result {
            super::damage::DamageResult::Hit(damage) => {
                def_health.current_hp -= damage;
                debug!(
                    attacker = %atk_id.name,
                    target = %def_id.name,
                    damage,
                    hp_remaining = def_health.current_hp,
                    "Melee hit"
                );
            }
            super::damage::DamageResult::Miss => {
                debug!(attacker = %atk_id.name, target = %def_id.name, "Melee miss");
            }
        }

        melee.reset_timer();
    }
}

pub fn system_process_death(
    mut commands: Commands,
    query: Query<(Entity, &Identity, &Health), (Without<DeadMarker>, With<NpcTemplate>)>,
) {
    for (entity, identity, health) in query.iter() {
        if health.current_hp <= 0 {
            info!(
                name = %identity.name,
                entity_id = identity.entity_id,
                "Entity died"
            );
            commands.entity(entity).insert(DeadMarker);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn death_processing() {
        let mut world = World::new();

        let entity = world.spawn((
            Identity {
                entity_id: 1, kind: EntityKind::Npc,
                name: "dying_orc".to_string(), last_name: String::new(),
                race: 1, class_id: 1, level: 5, gender: 0, deity: 0,
            },
            Health {
                current_hp: -10, max_hp: 100,
                current_mana: 0, max_mana: 0,
                current_endurance: 0, max_endurance: 0,
            },
            NpcTemplate { npc_type_id: 1, body_type: 1, animation: 0 },
        )).id();

        let mut schedule = Schedule::default();
        schedule.add_systems(system_process_death);
        schedule.run(&mut world);

        assert!(world.get::<DeadMarker>(entity).is_some());
    }

    #[test]
    fn alive_not_marked_dead() {
        let mut world = World::new();

        let entity = world.spawn((
            Identity {
                entity_id: 1, kind: EntityKind::Npc,
                name: "healthy_orc".to_string(), last_name: String::new(),
                race: 1, class_id: 1, level: 5, gender: 0, deity: 0,
            },
            Health {
                current_hp: 100, max_hp: 100,
                current_mana: 0, max_mana: 0,
                current_endurance: 0, max_endurance: 0,
            },
            NpcTemplate { npc_type_id: 1, body_type: 1, animation: 0 },
        )).id();

        let mut schedule = Schedule::default();
        schedule.add_systems(system_process_death);
        schedule.run(&mut world);

        assert!(world.get::<DeadMarker>(entity).is_none());
    }
}
