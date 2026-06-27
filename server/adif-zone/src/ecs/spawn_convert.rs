use adif_proto::adif;
use bevy_ecs::prelude::*;

use super::components::*;

pub fn build_spawn(
    identity: &Identity,
    position: &Position,
    velocity: &Velocity,
    health: &Health,
    movement: &MovementSpeed,
    model: &ModelAppearance,
    flags: &EntityFlags,
    appearance: Option<&PlayerAppearance>,
    guild: Option<&GuildMembership>,
    pet_owner: Option<&PetOwner>,
    npc_template: Option<&NpcTemplate>,
) -> adif::Spawn {
    adif::Spawn {
        entity_id: identity.entity_id,
        entity_type: entity_kind_to_proto(identity.kind) as i32,
        name: identity.name.clone(),
        last_name: identity.last_name.clone(),
        race: identity.race,
        class_id: identity.class_id,
        level: identity.level,
        gender: identity.gender,
        deity: identity.deity,
        position: Some(adif::Vec3 {
            x: position.x,
            y: position.y,
            z: position.z,
        }),
        heading: position.heading,
        velocity: Some(adif::Vec3 {
            x: velocity.x,
            y: velocity.y,
            z: velocity.z,
        }),
        current_hp: health.current_hp,
        max_hp: health.max_hp,
        appearance: appearance.map(|a| adif::Appearance {
            hair_color: a.hair_color,
            beard_color: a.beard_color,
            eye_color_1: a.eye_color_1,
            eye_color_2: a.eye_color_2,
            hair_style: a.hair_style,
            beard_style: a.beard_style,
            face: a.face,
            skin_tint: None,
        }),
        equipment: Vec::new(),
        gm: flags.gm,
        afk: flags.afk,
        anonymous: flags.anonymous,
        lfg: flags.lfg,
        sneaking: flags.sneaking,
        pvp: flags.pvp,
        linkdead: flags.linkdead,
        guild_id: guild.map_or(0, |g| g.guild_id),
        guild_rank: guild.map_or(0, |g| g.guild_rank),
        pet_owner_id: pet_owner.map_or(0, |p| p.owner_entity_id),
        body_type: npc_template.map_or(0, |t| t.body_type),
        animation: npc_template.map_or(0, |t| t.animation),
        run_speed: movement.run_speed,
        walk_speed: movement.walk_speed,
        size: model.size,
        light_source: model.light_source,
        texture: model.texture,
        helm_texture: model.helm_texture,
        invis: flags.invis,
        findable: flags.findable,
        show_helm: flags.show_helm,
        fly_mode: movement.fly_mode,
        title: String::new(),
        suffix: String::new(),
        bounding_radius: model.bounding_radius,
        is_pet: flags.is_pet,
        player_state: 0,
    }
}

pub fn build_spawn_from_world(world: &World, entity: Entity) -> Option<adif::Spawn> {
    let identity = world.get::<Identity>(entity)?;
    let position = world.get::<Position>(entity)?;
    let velocity = world.get::<Velocity>(entity);
    let health = world.get::<Health>(entity)?;
    let movement = world.get::<MovementSpeed>(entity);
    let model = world.get::<ModelAppearance>(entity);
    let flags = world.get::<EntityFlags>(entity);
    let appearance = world.get::<PlayerAppearance>(entity);
    let guild = world.get::<GuildMembership>(entity);
    let pet_owner = world.get::<PetOwner>(entity);
    let npc_template = world.get::<NpcTemplate>(entity);

    let default_velocity = Velocity::default();
    let default_movement = MovementSpeed::default();
    let default_model = ModelAppearance::default();
    let default_flags = EntityFlags::default();

    Some(build_spawn(
        identity,
        position,
        velocity.unwrap_or(&default_velocity),
        health,
        movement.unwrap_or(&default_movement),
        model.unwrap_or(&default_model),
        flags.unwrap_or(&default_flags),
        appearance,
        guild,
        pet_owner,
        npc_template,
    ))
}

pub fn build_position_update(
    identity: &Identity,
    position: &Position,
    velocity: &Velocity,
    animation: u32,
) -> adif::PositionUpdate {
    adif::PositionUpdate {
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
        animation,
    }
}

fn entity_kind_to_proto(kind: EntityKind) -> adif::EntityType {
    match kind {
        EntityKind::Player => adif::EntityType::Player,
        EntityKind::Npc => adif::EntityType::Npc,
        EntityKind::PlayerCorpse => adif::EntityType::PlayerCorpse,
        EntityKind::NpcCorpse => adif::EntityType::NpcCorpse,
    }
}

fn proto_to_entity_kind(proto: adif::EntityType) -> EntityKind {
    match proto {
        adif::EntityType::Player => EntityKind::Player,
        adif::EntityType::Npc => EntityKind::Npc,
        adif::EntityType::PlayerCorpse => EntityKind::PlayerCorpse,
        adif::EntityType::NpcCorpse => EntityKind::NpcCorpse,
        adif::EntityType::Unspecified => EntityKind::Npc,
    }
}

pub fn spawn_to_components(
    spawn: &adif::Spawn,
) -> (Identity, Position, Velocity, Health, MovementSpeed, ModelAppearance, EntityFlags) {
    let pos = spawn.position.as_ref();
    let vel = spawn.velocity.as_ref();

    let identity = Identity {
        entity_id: spawn.entity_id,
        kind: proto_to_entity_kind(adif::EntityType::try_from(spawn.entity_type).unwrap_or(adif::EntityType::Unspecified)),
        name: spawn.name.clone(),
        last_name: spawn.last_name.clone(),
        race: spawn.race,
        class_id: spawn.class_id,
        level: spawn.level,
        gender: spawn.gender,
        deity: spawn.deity,
    };

    let position = Position {
        x: pos.map_or(0.0, |p| p.x),
        y: pos.map_or(0.0, |p| p.y),
        z: pos.map_or(0.0, |p| p.z),
        heading: spawn.heading,
    };

    let velocity = Velocity {
        x: vel.map_or(0.0, |v| v.x),
        y: vel.map_or(0.0, |v| v.y),
        z: vel.map_or(0.0, |v| v.z),
        heading_delta: 0.0,
    };

    let health = Health {
        current_hp: spawn.current_hp,
        max_hp: spawn.max_hp,
        current_mana: 0,
        max_mana: 0,
        current_endurance: 0,
        max_endurance: 0,
    };

    let movement = MovementSpeed {
        run_speed: spawn.run_speed,
        walk_speed: spawn.walk_speed,
        fly_mode: spawn.fly_mode,
    };

    let model = ModelAppearance {
        size: spawn.size,
        light_source: spawn.light_source,
        texture: spawn.texture,
        helm_texture: spawn.helm_texture,
        bounding_radius: spawn.bounding_radius,
    };

    let flags = EntityFlags {
        gm: spawn.gm,
        afk: spawn.afk,
        anonymous: spawn.anonymous,
        lfg: spawn.lfg,
        sneaking: spawn.sneaking,
        pvp: spawn.pvp,
        linkdead: spawn.linkdead,
        invis: spawn.invis,
        findable: spawn.findable,
        show_helm: spawn.show_helm,
        is_pet: spawn.is_pet,
    };

    (identity, position, velocity, health, movement, model, flags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_npc_spawn() {
        let identity = Identity {
            entity_id: 42,
            kind: EntityKind::Npc,
            name: "a_fire_beetle".to_string(),
            last_name: String::new(),
            race: 11,
            class_id: 1,
            level: 3,
            gender: 0,
            deity: 0,
        };
        let position = Position { x: 100.0, y: 200.0, z: -5.0, heading: 90.0 };
        let velocity = Velocity::default();
        let health = Health {
            current_hp: 25, max_hp: 25,
            current_mana: 0, max_mana: 0,
            current_endurance: 0, max_endurance: 0,
        };
        let movement = MovementSpeed { run_speed: 0.7, walk_speed: 0.46, fly_mode: 0 };
        let model = ModelAppearance { size: 1.0, light_source: 0, texture: 0, helm_texture: 0, bounding_radius: 0.0 };
        let flags = EntityFlags::default();
        let npc_template = NpcTemplate { npc_type_id: 1234, body_type: 1, animation: 0 };

        let spawn = build_spawn(
            &identity, &position, &velocity, &health,
            &movement, &model, &flags,
            None, None, None, Some(&npc_template),
        );

        assert_eq!(spawn.entity_id, 42);
        assert_eq!(spawn.name, "a_fire_beetle");
        assert_eq!(spawn.level, 3);
        assert_eq!(spawn.entity_type, adif::EntityType::Npc as i32);
        assert_eq!(spawn.current_hp, 25);
        assert_eq!(spawn.body_type, 1);

        let pos = spawn.position.as_ref().unwrap();
        assert!((pos.x - 100.0).abs() < f32::EPSILON);
        assert!((pos.y - 200.0).abs() < f32::EPSILON);

        let (rt_id, rt_pos, _rt_vel, rt_health, _rt_move, _rt_model, _rt_flags) =
            spawn_to_components(&spawn);

        assert_eq!(rt_id.entity_id, 42);
        assert_eq!(rt_id.name, "a_fire_beetle");
        assert_eq!(rt_id.kind, EntityKind::Npc);
        assert!((rt_pos.x - 100.0).abs() < f32::EPSILON);
        assert_eq!(rt_health.current_hp, 25);
    }

    #[test]
    fn round_trip_player_spawn() {
        let identity = Identity {
            entity_id: 1,
            kind: EntityKind::Player,
            name: "Ghouldan".to_string(),
            last_name: "the Necromancer".to_string(),
            race: 5,
            class_id: 11,
            level: 50,
            gender: 0,
            deity: 201,
        };
        let position = Position { x: -50.5, y: 300.2, z: 10.0, heading: 180.0 };
        let velocity = Velocity { x: 1.0, y: 0.5, z: 0.0, heading_delta: 0.0 };
        let health = Health {
            current_hp: 2500, max_hp: 3000,
            current_mana: 1500, max_mana: 2000,
            current_endurance: 100, max_endurance: 100,
        };
        let movement = MovementSpeed::default();
        let model = ModelAppearance { size: 6.0, ..Default::default() };
        let flags = EntityFlags { show_helm: true, findable: true, ..Default::default() };
        let appearance = PlayerAppearance { hair_color: 2, face: 5, ..Default::default() };
        let guild = GuildMembership { guild_id: 99, guild_rank: 2 };

        let spawn = build_spawn(
            &identity, &position, &velocity, &health,
            &movement, &model, &flags,
            Some(&appearance), Some(&guild), None, None,
        );

        assert_eq!(spawn.entity_type, adif::EntityType::Player as i32);
        assert_eq!(spawn.name, "Ghouldan");
        assert_eq!(spawn.guild_id, 99);
        assert_eq!(spawn.guild_rank, 2);
        assert!(spawn.show_helm);
        assert!(spawn.findable);
        assert!(!spawn.gm);

        let app = spawn.appearance.as_ref().unwrap();
        assert_eq!(app.hair_color, 2);
        assert_eq!(app.face, 5);
    }

    #[test]
    fn build_spawn_from_ecs_world() {
        let mut world = World::new();
        let entity = world.spawn(NpcBundle {
            identity: Identity {
                entity_id: 7,
                kind: EntityKind::Npc,
                name: "Guard_Orcslayer".to_string(),
                last_name: String::new(),
                race: 1, class_id: 1, level: 30, gender: 0, deity: 0,
            },
            position: Position { x: 10.0, y: 20.0, z: 3.0, heading: 0.0 },
            velocity: Velocity::default(),
            health: Health {
                current_hp: 5000, max_hp: 5000,
                current_mana: 0, max_mana: 0,
                current_endurance: 0, max_endurance: 0,
            },
            movement: MovementSpeed::default(),
            model: ModelAppearance { size: 6.0, ..Default::default() },
            flags: EntityFlags::default(),
            npc_template: NpcTemplate { npc_type_id: 100, body_type: 1, animation: 0 },
        }).id();

        let spawn = build_spawn_from_world(&world, entity).unwrap();
        assert_eq!(spawn.entity_id, 7);
        assert_eq!(spawn.name, "Guard_Orcslayer");
        assert_eq!(spawn.level, 30);
        assert_eq!(spawn.current_hp, 5000);
    }

    #[test]
    fn position_update_conversion() {
        let identity = Identity {
            entity_id: 5,
            kind: EntityKind::Player,
            name: "Test".to_string(),
            last_name: String::new(),
            race: 1, class_id: 1, level: 1, gender: 0, deity: 0,
        };
        let position = Position { x: 1.0, y: 2.0, z: 3.0, heading: 45.0 };
        let velocity = Velocity { x: 0.5, y: 0.0, z: 0.0, heading_delta: 1.0 };

        let update = build_position_update(&identity, &position, &velocity, 10);
        assert_eq!(update.entity_id, 5);
        assert_eq!(update.animation, 10);
        assert!((update.heading - 45.0).abs() < f32::EPSILON);
        assert!((update.heading_delta - 1.0).abs() < f32::EPSILON);
    }
}
