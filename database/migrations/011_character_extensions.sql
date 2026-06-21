-- 011_character_extensions.sql
-- Character satellite tables for bind points, currencies, skills, etc.
--
-- EQEmu: Each of these is a separate table linked by character ID.
-- ADIF: Same pattern. Some data that EQEmu stores in flat columns on
-- character_data is stored in JSONB on ADIF's characters table, but these
-- satellite tables are new data the characters table doesn't cover.

-- Character bind/recall locations (home, secondary, etc.)
-- EQEmu: character_bind (id, slot, zone_id, x, y, z, heading).
-- ADIF characters table has bind_points JSONB, but the C++ code queries
-- this table directly — kept as separate table for C++ compatibility.
CREATE TABLE IF NOT EXISTS character_bind (
    id       INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    slot     SMALLINT NOT NULL DEFAULT 0,
    zone_id  INTEGER NOT NULL DEFAULT 0,
    x        REAL NOT NULL DEFAULT 0,
    y        REAL NOT NULL DEFAULT 0,
    z        REAL NOT NULL DEFAULT 0,
    heading  REAL NOT NULL DEFAULT 0,
    PRIMARY KEY (id, slot)
);

-- Alternate currencies (radiant/ebon crystals, etc.)
-- EQEmu: character_currency (id + many currency columns). Classic era only
-- needs radiant/ebon crystals.
CREATE TABLE IF NOT EXISTS character_currency (
    id                INTEGER PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    radiant_crystals  INTEGER NOT NULL DEFAULT 0,
    ebon_crystals     INTEGER NOT NULL DEFAULT 0
);

-- Alternate Advancement abilities earned by character
-- EQEmu: character_alternate_abilities (id, aa_id, aa_value, charges).
CREATE TABLE IF NOT EXISTS character_alternate_abilities (
    id      INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    aa_id   INTEGER NOT NULL DEFAULT 0,
    aa_value SMALLINT NOT NULL DEFAULT 0,
    charges SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id, aa_id)
);

-- Per-character faction standing values
-- EQEmu: character_faction_values (id, faction_id, current_value, temp).
CREATE TABLE IF NOT EXISTS character_faction_values (
    id            INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    faction_id    INTEGER NOT NULL DEFAULT 0,
    current_value INTEGER NOT NULL DEFAULT 0,
    temp          SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id, faction_id)
);

-- Language skill levels per character
-- EQEmu: character_languages (id, lang_id, value).
CREATE TABLE IF NOT EXISTS character_languages (
    id      INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    lang_id SMALLINT NOT NULL DEFAULT 0,
    value   SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id, lang_id)
);

-- Armor tinting/material per slot
-- EQEmu: character_material (id, slot, blue, green, red, use_tint, color).
CREATE TABLE IF NOT EXISTS character_material (
    id       INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    slot     SMALLINT NOT NULL DEFAULT 0,
    blue     SMALLINT NOT NULL DEFAULT 0,
    green    SMALLINT NOT NULL DEFAULT 0,
    red      SMALLINT NOT NULL DEFAULT 0,
    use_tint SMALLINT NOT NULL DEFAULT 0,
    color    INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id, slot)
);

-- Player inspect window message
-- EQEmu: character_inspect_messages (id, inspect_message).
CREATE TABLE IF NOT EXISTS character_inspect_messages (
    id              INTEGER PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    inspect_message TEXT NOT NULL DEFAULT ''
);

-- Zone access flags (keyed zones)
-- EQEmu: character_zone_flags (id, zoneID, key).
CREATE TABLE IF NOT EXISTS character_zone_flags (
    id      INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    "zoneID" INTEGER NOT NULL DEFAULT 0,
    "key"   SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id, "zoneID")
);

-- Reuse/cooldown timers
-- EQEmu: character_timers (id, type, start, duration, enable).
CREATE TABLE IF NOT EXISTS character_timers (
    id       INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    type     INTEGER NOT NULL DEFAULT 0,
    start    INTEGER NOT NULL DEFAULT 0,
    duration INTEGER NOT NULL DEFAULT 0,
    enable   BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (id, type)
);
