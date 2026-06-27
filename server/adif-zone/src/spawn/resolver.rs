use std::collections::HashMap;

use bevy_ecs::prelude::*;
use sqlx::PgPool;
use tracing::{info, warn};

use crate::ecs::components::*;
use crate::ecs::EntityIdAllocator;
use super::npc_type::NpcType;
use super::spawn_point::{SpawnEntry, SpawnPoint};

pub struct SpawnResult {
    pub npcs_spawned: usize,
    pub spawn_points_total: usize,
    pub spawn_points_empty: usize,
}

pub async fn load_and_spawn(
    pool: &PgPool,
    zone_short_name: &str,
    world: &mut World,
) -> anyhow::Result<SpawnResult> {
    let spawn_points = SpawnPoint::load_for_zone(pool, zone_short_name).await?;
    let spawn_entries = SpawnEntry::load_for_zone(pool, zone_short_name).await?;
    let npc_types = NpcType::load_for_zone(pool, zone_short_name).await?;

    let npc_map: HashMap<i32, &NpcType> = npc_types.iter().map(|n| (n.id, n)).collect();

    let mut entries_by_group: HashMap<i32, Vec<&SpawnEntry>> = HashMap::new();
    for entry in &spawn_entries {
        entries_by_group
            .entry(entry.spawngroupid)
            .or_default()
            .push(entry);
    }

    info!(
        spawn_points = spawn_points.len(),
        npc_types = npc_types.len(),
        spawn_entries = spawn_entries.len(),
        "Loaded spawn data for zone"
    );

    let mut npcs_spawned = 0;
    let mut spawn_points_empty = 0;
    let spawn_points_total = spawn_points.len();

    for point in &spawn_points {
        let entries = match entries_by_group.get(&point.spawngroupid) {
            Some(e) => e,
            None => {
                spawn_points_empty += 1;
                continue;
            }
        };

        let npc_id = pick_npc(entries);

        let npc = match npc_id.and_then(|id| npc_map.get(&id)) {
            Some(n) => n,
            None => {
                spawn_points_empty += 1;
                continue;
            }
        };

        let ecs_entity = world.spawn_empty().id();
        let entity_id = world.resource_mut::<EntityIdAllocator>().allocate(ecs_entity);

        world.entity_mut(ecs_entity).insert(NpcBundle {
            identity: Identity {
                entity_id,
                kind: EntityKind::Npc,
                name: npc.name.clone(),
                last_name: npc.lastname.clone().unwrap_or_default(),
                race: npc.race as u32,
                class_id: npc.class as u32,
                level: npc.level as u32,
                gender: npc.gender as u32,
                deity: 0,
            },
            position: Position {
                x: point.x,
                y: point.y,
                z: point.z,
                heading: point.heading,
            },
            velocity: Velocity::default(),
            health: Health {
                current_hp: npc.hp as i32,
                max_hp: npc.hp as i32,
                current_mana: npc.mana as i32,
                max_mana: npc.mana as i32,
                current_endurance: 0,
                max_endurance: 0,
            },
            movement: MovementSpeed {
                run_speed: if npc.runspeed > 0.0 { npc.runspeed } else { 0.7 },
                walk_speed: if npc.walkspeed > 0.0 { npc.walkspeed } else { 0.46 },
                fly_mode: npc.flymode.max(0) as u32,
            },
            model: ModelAppearance {
                size: npc.size,
                light_source: npc.light as u32,
                texture: npc.texture as u32,
                helm_texture: npc.helmtexture as u32,
                bounding_radius: 0.0,
            },
            flags: EntityFlags {
                findable: npc.findable != 0,
                show_helm: true,
                ..Default::default()
            },
            npc_template: NpcTemplate {
                npc_type_id: npc.id,
                body_type: npc.bodytype as u32,
                animation: point.animation as u32,
            },
        });

        npcs_spawned += 1;
    }

    if spawn_points_empty > 0 {
        warn!(
            empty = spawn_points_empty,
            "Spawn points with no valid NPC entries"
        );
    }

    Ok(SpawnResult {
        npcs_spawned,
        spawn_points_total,
        spawn_points_empty,
    })
}

fn pick_npc(entries: &[&SpawnEntry]) -> Option<i32> {
    if entries.is_empty() {
        return None;
    }
    if entries.len() == 1 {
        return Some(entries[0].npcid);
    }

    let total_chance: i32 = entries.iter().map(|e| e.chance).sum();
    if total_chance <= 0 {
        return Some(entries[0].npcid);
    }

    // Deterministic pick: take the highest-chance entry
    // (random selection will come with the game loop tick system)
    entries
        .iter()
        .max_by_key(|e| e.chance)
        .map(|e| e.npcid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_npc_single_entry() {
        let entry = SpawnEntry { spawngroupid: 1, npcid: 100, chance: 100 };
        let entries = vec![&entry];
        assert_eq!(pick_npc(&entries), Some(100));
    }

    #[test]
    fn pick_npc_highest_chance() {
        let e1 = SpawnEntry { spawngroupid: 1, npcid: 100, chance: 30 };
        let e2 = SpawnEntry { spawngroupid: 1, npcid: 200, chance: 70 };
        let entries = vec![&e1, &e2];
        assert_eq!(pick_npc(&entries), Some(200));
    }

    #[test]
    fn pick_npc_empty() {
        let entries: Vec<&SpawnEntry> = vec![];
        assert_eq!(pick_npc(&entries), None);
    }
}
