-- 015_world_objects.sql
-- World objects, ground spawns, traps, graveyards, and spawn condition system.
--
-- EQEmu: object, ground_spawns, traps, graveyard, blocked_spells,
-- spawn_conditions, spawn_condition_values, spawn_events, respawn_times.
-- ADIF: Identical — zone-level content placement.

-- World objects (forges, looms, tradeskill containers, etc.)
-- EQEmu: object (18 columns). Unchanged.
CREATE TABLE IF NOT EXISTS object (
    id                      SERIAL PRIMARY KEY,
    zoneid                  INTEGER NOT NULL DEFAULT 0,
    xpos                    REAL NOT NULL DEFAULT 0,
    ypos                    REAL NOT NULL DEFAULT 0,
    zpos                    REAL NOT NULL DEFAULT 0,
    heading                 REAL NOT NULL DEFAULT 0,
    itemid                  INTEGER NOT NULL DEFAULT 0,
    charges                 SMALLINT NOT NULL DEFAULT 0,
    objectname              TEXT NOT NULL DEFAULT '',
    type                    INTEGER NOT NULL DEFAULT 0,
    icon                    INTEGER NOT NULL DEFAULT 0,
    size                    INTEGER NOT NULL DEFAULT 0,
    solid                   INTEGER NOT NULL DEFAULT 0,
    incline                 INTEGER NOT NULL DEFAULT 0,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_object_zoneid ON object(zoneid);

-- Items inside world objects (forge contents, etc.)
-- EQEmu: object_contents. Unchanged.
CREATE TABLE IF NOT EXISTS object_contents (
    zoneid   INTEGER NOT NULL DEFAULT 0,
    parentid INTEGER NOT NULL DEFAULT 0,
    bagidx   INTEGER NOT NULL DEFAULT 0,
    itemid   INTEGER NOT NULL DEFAULT 0,
    charges  SMALLINT NOT NULL DEFAULT 0,
    droptime TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (zoneid, parentid, bagidx)
);

-- Ground-spawned items (collectibles, quest items on the ground)
-- EQEmu: ground_spawns (17 columns). Unchanged.
CREATE TABLE IF NOT EXISTS ground_spawns (
    id                      SERIAL PRIMARY KEY,
    zoneid                  INTEGER NOT NULL DEFAULT 0,
    max_x                   REAL NOT NULL DEFAULT 0,
    max_y                   REAL NOT NULL DEFAULT 0,
    max_z                   REAL NOT NULL DEFAULT 0,
    min_x                   REAL NOT NULL DEFAULT 0,
    min_y                   REAL NOT NULL DEFAULT 0,
    heading                 REAL NOT NULL DEFAULT 0,
    name                    TEXT NOT NULL DEFAULT '',
    item                    INTEGER NOT NULL DEFAULT 0,
    max_allowed             INTEGER NOT NULL DEFAULT 1,
    comment                 TEXT NOT NULL DEFAULT '',
    respawn_timer           BIGINT NOT NULL DEFAULT 300,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_ground_spawns_zone ON ground_spawns(zoneid);

-- Death return points (where you respawn after dying)
-- EQEmu: graveyard. Unchanged.
CREATE TABLE IF NOT EXISTS graveyard (
    id       SERIAL PRIMARY KEY,
    zone_id  INTEGER NOT NULL DEFAULT 0,
    x        REAL NOT NULL DEFAULT 0,
    y        REAL NOT NULL DEFAULT 0,
    z        REAL NOT NULL DEFAULT 0,
    heading  REAL NOT NULL DEFAULT 0
);

-- Zone traps (damage/debuff areas)
-- EQEmu: traps (24 columns). Unchanged.
CREATE TABLE IF NOT EXISTS traps (
    id                      SERIAL PRIMARY KEY,
    zone                    TEXT NOT NULL DEFAULT '',
    x                       INTEGER NOT NULL DEFAULT 0,
    y                       INTEGER NOT NULL DEFAULT 0,
    z                       INTEGER NOT NULL DEFAULT 0,
    chance                  SMALLINT NOT NULL DEFAULT 0,
    maxzdiff                REAL NOT NULL DEFAULT 0,
    radius                  REAL NOT NULL DEFAULT 0,
    effect                  INTEGER NOT NULL DEFAULT 0,
    effectvalue             INTEGER NOT NULL DEFAULT 0,
    effectvalue2            INTEGER NOT NULL DEFAULT 0,
    message                 TEXT NOT NULL DEFAULT '',
    skill                   INTEGER NOT NULL DEFAULT 0,
    level                   INTEGER NOT NULL DEFAULT 0,
    respawn_time            INTEGER NOT NULL DEFAULT 60,
    respawn_var             INTEGER NOT NULL DEFAULT 0,
    triggered_number        SMALLINT NOT NULL DEFAULT 0,
    "group"                 SMALLINT NOT NULL DEFAULT 0,
    despawn_when_triggered  SMALLINT NOT NULL DEFAULT 0,
    undetectable            SMALLINT NOT NULL DEFAULT 0,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);

-- Spells blocked in specific zones
-- EQEmu: blocked_spells (14 columns). Unchanged.
CREATE TABLE IF NOT EXISTS blocked_spells (
    id                      SERIAL PRIMARY KEY,
    spellid                 INTEGER NOT NULL DEFAULT 0,
    type                    SMALLINT NOT NULL DEFAULT 0,
    zoneid                  INTEGER NOT NULL DEFAULT 0,
    x                       REAL NOT NULL DEFAULT 0,
    y                       REAL NOT NULL DEFAULT 0,
    z                       REAL NOT NULL DEFAULT 0,
    x_diff                  REAL NOT NULL DEFAULT 0,
    y_diff                  REAL NOT NULL DEFAULT 0,
    z_diff                  REAL NOT NULL DEFAULT 0,
    message                 TEXT NOT NULL DEFAULT '',
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);

-- Conditional spawn rules (weather, time-of-day, quest-triggered)
-- EQEmu: spawn_conditions. Unchanged.
CREATE TABLE IF NOT EXISTS spawn_conditions (
    zone     TEXT NOT NULL DEFAULT '',
    id       INTEGER NOT NULL DEFAULT 0,
    value    INTEGER NOT NULL DEFAULT 0,
    onchange SMALLINT NOT NULL DEFAULT 0,
    name     TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (zone, id)
);

-- Runtime spawn condition values
-- EQEmu: spawn_condition_values. Unchanged.
CREATE TABLE IF NOT EXISTS spawn_condition_values (
    id    INTEGER NOT NULL DEFAULT 0,
    zone  TEXT NOT NULL DEFAULT '',
    value SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id, zone)
);

-- Timed spawn events (holiday spawns, scheduled content)
-- EQEmu: spawn_events. Unchanged.
CREATE TABLE IF NOT EXISTS spawn_events (
    id            SERIAL PRIMARY KEY,
    zone          TEXT NOT NULL DEFAULT '',
    cond_id       INTEGER NOT NULL DEFAULT 0,
    name          TEXT NOT NULL DEFAULT '',
    period        INTEGER NOT NULL DEFAULT 0,
    next_minute   SMALLINT NOT NULL DEFAULT 0,
    next_hour     SMALLINT NOT NULL DEFAULT 0,
    next_day      SMALLINT NOT NULL DEFAULT 0,
    next_month    SMALLINT NOT NULL DEFAULT 0,
    next_year     INTEGER NOT NULL DEFAULT 0,
    enabled       BOOLEAN NOT NULL DEFAULT TRUE,
    action        SMALLINT NOT NULL DEFAULT 0,
    argument      INTEGER NOT NULL DEFAULT 0,
    strict        BOOLEAN NOT NULL DEFAULT FALSE
);

-- NPC respawn timers (runtime tracking)
-- EQEmu: respawn_times. Unchanged.
CREATE TABLE IF NOT EXISTS respawn_times (
    id       INTEGER PRIMARY KEY,
    start    INTEGER NOT NULL DEFAULT 0,
    duration INTEGER NOT NULL DEFAULT 0
);
