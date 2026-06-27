use bevy_ecs::prelude::*;

use crate::ecs::components::*;
use crate::network::broadcast::distance_2d;
use super::state::{AiBrain, AiState};
use super::hate_list::HateList;

pub fn system_ai_aggro_check(
    mut npcs: Query<(&Identity, &Position, &mut AiBrain, &mut HateList), With<NpcTemplate>>,
    players: Query<(&Identity, &Position), Without<NpcTemplate>>,
) {
    for (npc_id, npc_pos, mut brain, mut hate_list) in npcs.iter_mut() {
        if brain.state != AiState::Idle && brain.state != AiState::Patrol {
            continue;
        }

        for (player_id, player_pos) in players.iter() {
            let dist = distance_2d(npc_pos.x, npc_pos.y, player_pos.x, player_pos.y);
            if dist <= brain.aggro_radius {
                hate_list.add_hate(player_id.entity_id, 1);
                brain.state = AiState::Aggro;
                break;
            }
        }
    }
}

pub fn system_ai_chase(
    mut npcs: Query<(&Identity, &mut Position, &mut AiBrain, &HateList), With<NpcTemplate>>,
    targets: Query<(&Identity, &Position)>,
) {
    for (_npc_id, mut npc_pos, mut brain, hate_list) in npcs.iter_mut() {
        if brain.state != AiState::Aggro && brain.state != AiState::Chase {
            continue;
        }

        let target_id = match hate_list.top_target() {
            Some(id) => id,
            None => {
                brain.state = AiState::Leash;
                continue;
            }
        };

        let target_pos = targets
            .iter()
            .find(|(id, _)| id.entity_id == target_id)
            .map(|(_, pos)| *pos);

        let target_pos = match target_pos {
            Some(p) => p,
            None => {
                brain.state = AiState::Leash;
                continue;
            }
        };

        if brain.should_leash(npc_pos.x, npc_pos.y) {
            brain.state = AiState::Leash;
            continue;
        }

        let dist = distance_2d(npc_pos.x, npc_pos.y, target_pos.x, target_pos.y);
        brain.state = if dist <= 15.0 { AiState::Combat } else { AiState::Chase };

        if brain.state == AiState::Chase {
            let dx = target_pos.x - npc_pos.x;
            let dy = target_pos.y - npc_pos.y;
            let len = (dx * dx + dy * dy).sqrt();
            if len > 0.01 {
                let speed = 0.7 * 32.0 / 1000.0 * 20.0;
                npc_pos.x += (dx / len) * speed;
                npc_pos.y += (dy / len) * speed;
            }
        }
    }
}

pub fn system_ai_leash(
    mut npcs: Query<(&mut Position, &mut AiBrain, &mut HateList), With<NpcTemplate>>,
) {
    for (mut pos, mut brain, mut hate_list) in npcs.iter_mut() {
        if brain.state != AiState::Leash {
            continue;
        }

        hate_list.clear();
        pos.x = brain.home_x;
        pos.y = brain.home_y;
        pos.z = brain.home_z;
        brain.state = if brain.patrol_grid_id > 0 { AiState::Patrol } else { AiState::Idle };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggro_check_triggers_on_close_player() {
        let mut world = World::new();

        world.spawn((
            Identity {
                entity_id: 1, kind: EntityKind::Npc,
                name: "orc".to_string(), last_name: String::new(),
                race: 1, class_id: 1, level: 5, gender: 0, deity: 0,
            },
            Position { x: 100.0, y: 100.0, z: 0.0, heading: 0.0 },
            AiBrain::new(100.0, 100.0, 0.0, 70.0, 30.0, 0),
            HateList::default(),
            NpcTemplate { npc_type_id: 1, body_type: 1, animation: 0 },
        ));

        world.spawn((
            Identity {
                entity_id: 100, kind: EntityKind::Player,
                name: "Ghouldan".to_string(), last_name: String::new(),
                race: 5, class_id: 11, level: 50, gender: 0, deity: 0,
            },
            Position { x: 150.0, y: 100.0, z: 0.0, heading: 0.0 },
        ));

        let mut schedule = Schedule::default();
        schedule.add_systems(system_ai_aggro_check);
        schedule.run(&mut world);

        let mut query = world.query::<(&AiBrain, &HateList)>();
        for (brain, hate) in query.iter(&world) {
            assert_eq!(brain.state, AiState::Aggro);
            assert!(hate.contains(100));
        }
    }

    #[test]
    fn no_aggro_when_out_of_range() {
        let mut world = World::new();

        world.spawn((
            Identity {
                entity_id: 1, kind: EntityKind::Npc,
                name: "orc".to_string(), last_name: String::new(),
                race: 1, class_id: 1, level: 5, gender: 0, deity: 0,
            },
            Position { x: 100.0, y: 100.0, z: 0.0, heading: 0.0 },
            AiBrain::new(100.0, 100.0, 0.0, 70.0, 30.0, 0),
            HateList::default(),
            NpcTemplate { npc_type_id: 1, body_type: 1, animation: 0 },
        ));

        world.spawn((
            Identity {
                entity_id: 100, kind: EntityKind::Player,
                name: "Ghouldan".to_string(), last_name: String::new(),
                race: 5, class_id: 11, level: 50, gender: 0, deity: 0,
            },
            Position { x: 500.0, y: 500.0, z: 0.0, heading: 0.0 },
        ));

        let mut schedule = Schedule::default();
        schedule.add_systems(system_ai_aggro_check);
        schedule.run(&mut world);

        let mut query = world.query::<&AiBrain>();
        for brain in query.iter(&world) {
            assert_eq!(brain.state, AiState::Idle);
        }
    }

    #[test]
    fn leash_returns_home_and_clears_hate() {
        let mut world = World::new();

        world.spawn((
            Position { x: 900.0, y: 900.0, z: 0.0, heading: 0.0 },
            AiBrain { state: AiState::Leash, home_x: 100.0, home_y: 100.0, home_z: 0.0, aggro_radius: 70.0, assist_radius: 30.0, leash_radius: 600.0, patrol_grid_id: 0 },
            HateList::default(),
            NpcTemplate { npc_type_id: 1, body_type: 1, animation: 0 },
        ));

        let mut schedule = Schedule::default();
        schedule.add_systems(system_ai_leash);
        schedule.run(&mut world);

        let mut query = world.query::<(&Position, &AiBrain, &HateList)>();
        for (pos, brain, hate) in query.iter(&world) {
            assert!((pos.x - 100.0).abs() < f32::EPSILON);
            assert!((pos.y - 100.0).abs() < f32::EPSILON);
            assert_eq!(brain.state, AiState::Idle);
            assert!(hate.is_empty());
        }
    }
}
