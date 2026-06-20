-- NPC templates and spawning.
-- EQEmu: npc_types (99 columns) + spawn2 + spawngroup + spawnentry (4 tables).
-- ADIF: cleaner with JSONB for resistances/abilities, proper FKs.

CREATE TABLE npc_templates (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(64) NOT NULL,
    last_name       VARCHAR(64) NOT NULL DEFAULT '',

    -- Base stats
    level           SMALLINT NOT NULL DEFAULT 1,
    race            SMALLINT NOT NULL DEFAULT 0,
    class_id        SMALLINT NOT NULL DEFAULT 0,
    gender          SMALLINT NOT NULL DEFAULT 0,
    body_type       SMALLINT NOT NULL DEFAULT 0,

    -- Health & resources
    hp              INTEGER NOT NULL DEFAULT 100,
    mana            INTEGER NOT NULL DEFAULT 0,
    hp_regen        INTEGER NOT NULL DEFAULT 0,
    mana_regen      INTEGER NOT NULL DEFAULT 0,
    combat_hp_regen INTEGER NOT NULL DEFAULT 0,
    combat_mana_regen INTEGER NOT NULL DEFAULT 0,

    -- Combat
    min_damage      INTEGER NOT NULL DEFAULT 1,
    max_damage      INTEGER NOT NULL DEFAULT 10,
    attack_count    SMALLINT NOT NULL DEFAULT 1,
    attack_speed    REAL NOT NULL DEFAULT 0,
    aggro_radius    INTEGER NOT NULL DEFAULT 70,
    assist_radius   INTEGER NOT NULL DEFAULT 70,
    ac              SMALLINT NOT NULL DEFAULT 0,

    -- Attributes
    str             SMALLINT NOT NULL DEFAULT 75,
    sta             SMALLINT NOT NULL DEFAULT 75,
    dex             SMALLINT NOT NULL DEFAULT 75,
    agi             SMALLINT NOT NULL DEFAULT 75,
    int_            SMALLINT NOT NULL DEFAULT 75,
    wis             SMALLINT NOT NULL DEFAULT 75,
    cha             SMALLINT NOT NULL DEFAULT 75,

    -- Resistances (JSONB replaces 5 separate columns)
    resistances     JSONB NOT NULL DEFAULT '{"magic": 0, "fire": 0, "cold": 0, "disease": 0, "poison": 0}',

    -- Appearance
    size            REAL NOT NULL DEFAULT 1.0,
    texture         SMALLINT NOT NULL DEFAULT 0,
    helm_texture    SMALLINT NOT NULL DEFAULT 0,
    appearance      JSONB NOT NULL DEFAULT '{}',

    -- Movement
    run_speed       REAL NOT NULL DEFAULT 1.25,
    walk_speed      REAL NOT NULL DEFAULT 0.5,

    -- Perception
    see_invisible   BOOLEAN NOT NULL DEFAULT FALSE,
    see_invis_undead BOOLEAN NOT NULL DEFAULT FALSE,

    -- Special abilities (JSONB array)
    special_abilities JSONB NOT NULL DEFAULT '[]',

    -- References
    loot_table_id   INTEGER,
    merchant_id     INTEGER,
    spell_list_id   INTEGER,
    faction_id      INTEGER,

    -- Flags
    is_rare_spawn   BOOLEAN NOT NULL DEFAULT FALSE,
    trackable       BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX idx_npc_templates_name ON npc_templates (name);

-- Spawn groups: which NPCs can spawn at a location.
CREATE TABLE spawn_groups (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(128) NOT NULL DEFAULT '',
    spawn_limit     SMALLINT NOT NULL DEFAULT 0,
    despawn_timer   INTEGER NOT NULL DEFAULT 0
);

-- Spawn group entries: NPCs in a group with probability weights.
CREATE TABLE spawn_entries (
    spawn_group_id  INTEGER NOT NULL REFERENCES spawn_groups(id) ON DELETE CASCADE,
    npc_template_id INTEGER NOT NULL REFERENCES npc_templates(id) ON DELETE CASCADE,
    chance          SMALLINT NOT NULL DEFAULT 100,  -- percentage
    PRIMARY KEY (spawn_group_id, npc_template_id)
);

-- Spawn points: where in a zone things spawn.
CREATE TABLE spawn_points (
    id              SERIAL PRIMARY KEY,
    zone_id         INTEGER NOT NULL REFERENCES zones(id) ON DELETE CASCADE,
    spawn_group_id  INTEGER NOT NULL REFERENCES spawn_groups(id) ON DELETE CASCADE,

    -- Position
    x               REAL NOT NULL,
    y               REAL NOT NULL,
    z               REAL NOT NULL,
    heading         REAL NOT NULL DEFAULT 0,

    -- Timing
    respawn_time    INTEGER NOT NULL DEFAULT 360,   -- seconds
    variance        INTEGER NOT NULL DEFAULT 0,     -- seconds
    boot_respawn    INTEGER NOT NULL DEFAULT 0,     -- seconds (server start)

    -- Pathing
    patrol_grid_id  INTEGER,

    -- Behavior
    animation       SMALLINT NOT NULL DEFAULT 0,    -- 0=standing, 1=sitting, etc.
    enabled         BOOLEAN NOT NULL DEFAULT TRUE,
    force_z         BOOLEAN NOT NULL DEFAULT FALSE,

    -- Conditional spawning
    condition_id    INTEGER NOT NULL DEFAULT 0,
    condition_value INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_spawn_points_zone ON spawn_points (zone_id);

-- NPC patrol grids.
CREATE TABLE patrol_grids (
    id              SERIAL PRIMARY KEY,
    zone_id         INTEGER NOT NULL REFERENCES zones(id) ON DELETE CASCADE,
    wander_type     SMALLINT NOT NULL DEFAULT 0,    -- 0=none, 1=random, 2=patrol
    pause_type      SMALLINT NOT NULL DEFAULT 0     -- 0=none, 1=random
);

-- Patrol waypoints.
CREATE TABLE patrol_waypoints (
    grid_id         INTEGER NOT NULL REFERENCES patrol_grids(id) ON DELETE CASCADE,
    number          SMALLINT NOT NULL,
    x               REAL NOT NULL,
    y               REAL NOT NULL,
    z               REAL NOT NULL,
    heading         REAL NOT NULL DEFAULT 0,
    pause_ms        INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (grid_id, number)
);

CREATE INDEX idx_patrol_grids_zone ON patrol_grids (zone_id);
