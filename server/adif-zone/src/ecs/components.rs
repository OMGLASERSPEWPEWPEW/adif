use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading: f32,
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub heading_delta: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityKind {
    Player,
    Npc,
    PlayerCorpse,
    NpcCorpse,
}

#[derive(Component, Debug, Clone)]
pub struct Identity {
    pub entity_id: u32,
    pub kind: EntityKind,
    pub name: String,
    pub last_name: String,
    pub race: u32,
    pub class_id: u32,
    pub level: u32,
    pub gender: u32,
    pub deity: u32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    pub current_hp: i32,
    pub max_hp: i32,
    pub current_mana: i32,
    pub max_mana: i32,
    pub current_endurance: i32,
    pub max_endurance: i32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct MovementSpeed {
    pub run_speed: f32,
    pub walk_speed: f32,
    pub fly_mode: u32,
}

impl Default for MovementSpeed {
    fn default() -> Self {
        Self {
            run_speed: 0.7,
            walk_speed: 0.46,
            fly_mode: 0,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct ModelAppearance {
    pub size: f32,
    pub light_source: u32,
    pub texture: u32,
    pub helm_texture: u32,
    pub bounding_radius: f32,
}

#[derive(Component, Debug, Clone, Default)]
pub struct PlayerAppearance {
    pub hair_color: u32,
    pub beard_color: u32,
    pub eye_color_1: u32,
    pub eye_color_2: u32,
    pub hair_style: u32,
    pub beard_style: u32,
    pub face: u32,
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct EntityFlags {
    pub gm: bool,
    pub afk: bool,
    pub anonymous: bool,
    pub lfg: bool,
    pub sneaking: bool,
    pub pvp: bool,
    pub linkdead: bool,
    pub invis: bool,
    pub findable: bool,
    pub show_helm: bool,
    pub is_pet: bool,
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct GuildMembership {
    pub guild_id: u32,
    pub guild_rank: u32,
}

#[derive(Component, Debug, Clone)]
pub struct NpcTemplate {
    pub npc_type_id: i32,
    pub body_type: u32,
    pub animation: u32,
}

#[derive(Component, Debug)]
pub struct ClientSession {
    pub account_id: i32,
    pub character_id: i32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct PetOwner {
    pub owner_entity_id: u32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CombatTarget {
    pub target_entity_id: u32,
}

#[derive(Bundle)]
pub struct NpcBundle {
    pub identity: Identity,
    pub position: Position,
    pub velocity: Velocity,
    pub health: Health,
    pub movement: MovementSpeed,
    pub model: ModelAppearance,
    pub flags: EntityFlags,
    pub npc_template: NpcTemplate,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub identity: Identity,
    pub position: Position,
    pub velocity: Velocity,
    pub health: Health,
    pub movement: MovementSpeed,
    pub model: ModelAppearance,
    pub appearance: PlayerAppearance,
    pub flags: EntityFlags,
    pub guild: GuildMembership,
    pub session: ClientSession,
}
