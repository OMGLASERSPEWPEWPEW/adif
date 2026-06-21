-- 010_character_creation.sql
-- Tables required for character creation flow.
--
-- EQEmu: start_zones, char_create_combinations, char_create_point_allocations,
-- starting_items, level_exp_mods. ADIF: identical schema — these define the
-- character creation rules, not game content.

-- Starting zone assignments by race/class/deity
-- EQEmu: start_zones (20 columns). Unchanged.
CREATE TABLE IF NOT EXISTS start_zones (
    id                      SERIAL PRIMARY KEY,
    x                       REAL NOT NULL DEFAULT 0,
    y                       REAL NOT NULL DEFAULT 0,
    z                       REAL NOT NULL DEFAULT 0,
    heading                 REAL NOT NULL DEFAULT 0,
    zone_id                 INTEGER NOT NULL DEFAULT 0,
    bind_id                 INTEGER NOT NULL DEFAULT 0,
    player_choice           SMALLINT NOT NULL DEFAULT 0,
    player_class            SMALLINT NOT NULL DEFAULT 0,
    player_deity            SMALLINT NOT NULL DEFAULT 0,
    player_race             SMALLINT NOT NULL DEFAULT 0,
    start_zone              SMALLINT NOT NULL DEFAULT 0,
    bind_x                  REAL NOT NULL DEFAULT 0,
    bind_y                  REAL NOT NULL DEFAULT 0,
    bind_z                  REAL NOT NULL DEFAULT 0,
    select_rank             SMALLINT NOT NULL DEFAULT 0,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_start_zones_lookup ON start_zones(player_race, player_class, player_deity);

-- Valid race/class/deity combinations for character creation
-- EQEmu: char_create_combinations. Unchanged.
CREATE TABLE IF NOT EXISTS char_create_combinations (
    id              SERIAL PRIMARY KEY,
    race            SMALLINT NOT NULL DEFAULT 0,
    "class"         SMALLINT NOT NULL DEFAULT 0,
    deity           SMALLINT NOT NULL DEFAULT 0,
    start_zone      SMALLINT NOT NULL DEFAULT 0,
    allocation_id   INTEGER NOT NULL DEFAULT 0,
    expansion       SMALLINT NOT NULL DEFAULT 0
);

-- Base stat allocations for each race/class combination
-- EQEmu: char_create_point_allocations. Unchanged.
CREATE TABLE IF NOT EXISTS char_create_point_allocations (
    id          SERIAL PRIMARY KEY,
    base_str    SMALLINT NOT NULL DEFAULT 0,
    base_sta    SMALLINT NOT NULL DEFAULT 0,
    base_dex    SMALLINT NOT NULL DEFAULT 0,
    base_agi    SMALLINT NOT NULL DEFAULT 0,
    base_int    SMALLINT NOT NULL DEFAULT 0,
    base_wis    SMALLINT NOT NULL DEFAULT 0,
    base_cha    SMALLINT NOT NULL DEFAULT 0,
    alloc_str   SMALLINT NOT NULL DEFAULT 0,
    alloc_sta   SMALLINT NOT NULL DEFAULT 0,
    alloc_dex   SMALLINT NOT NULL DEFAULT 0,
    alloc_agi   SMALLINT NOT NULL DEFAULT 0,
    alloc_int   SMALLINT NOT NULL DEFAULT 0,
    alloc_wis   SMALLINT NOT NULL DEFAULT 0,
    alloc_cha   SMALLINT NOT NULL DEFAULT 0
);

-- Starting equipment given to new characters
-- EQEmu: starting_items. Unchanged.
CREATE TABLE IF NOT EXISTS starting_items (
    id                      SERIAL PRIMARY KEY,
    race                    INTEGER NOT NULL DEFAULT 0,
    "class"                 SMALLINT NOT NULL DEFAULT 0,
    deityid                 INTEGER NOT NULL DEFAULT 0,
    zoneid                  INTEGER NOT NULL DEFAULT 0,
    itemid                  INTEGER NOT NULL DEFAULT 0,
    item_charges            SMALLINT NOT NULL DEFAULT 0,
    gm                      BOOLEAN NOT NULL DEFAULT FALSE,
    slot                    SMALLINT NOT NULL DEFAULT -1,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);

-- Experience multipliers by level
-- EQEmu: level_exp_mods. Unchanged.
CREATE TABLE IF NOT EXISTS level_exp_mods (
    level   SMALLINT PRIMARY KEY,
    exp_mod REAL NOT NULL DEFAULT 1.0
);
