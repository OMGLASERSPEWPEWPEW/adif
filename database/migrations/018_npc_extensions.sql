-- 018_npc_extensions.sql
-- NPC system extensions: tinting, emotes, faction detail, spell effects.
--
-- EQEmu: npc_types_tint, npc_emotes, npc_faction, npc_faction_entries,
-- npc_spells_effects, npc_spells_effects_entries, faction_list, faction_list_mod,
-- damageshieldtypes, spell_globals.
-- ADIF: Identical — NPC behavior data is inherently tabular.

-- NPC armor tinting (color overrides per slot)
-- EQEmu: npc_types_tint. Unchanged.
CREATE TABLE IF NOT EXISTS npc_types_tint (
    id              SERIAL PRIMARY KEY,
    tint_set_name   TEXT NOT NULL DEFAULT '',
    red1h           SMALLINT NOT NULL DEFAULT 0,
    grn1h           SMALLINT NOT NULL DEFAULT 0,
    blu1h           SMALLINT NOT NULL DEFAULT 0,
    red2c           SMALLINT NOT NULL DEFAULT 0,
    grn2c           SMALLINT NOT NULL DEFAULT 0,
    blu2c           SMALLINT NOT NULL DEFAULT 0,
    red3a           SMALLINT NOT NULL DEFAULT 0,
    grn3a           SMALLINT NOT NULL DEFAULT 0,
    blu3a           SMALLINT NOT NULL DEFAULT 0,
    red4b           SMALLINT NOT NULL DEFAULT 0,
    grn4b           SMALLINT NOT NULL DEFAULT 0,
    blu4b           SMALLINT NOT NULL DEFAULT 0,
    red5g           SMALLINT NOT NULL DEFAULT 0,
    grn5g           SMALLINT NOT NULL DEFAULT 0,
    blu5g           SMALLINT NOT NULL DEFAULT 0,
    red6l           SMALLINT NOT NULL DEFAULT 0,
    grn6l           SMALLINT NOT NULL DEFAULT 0,
    blu6l           SMALLINT NOT NULL DEFAULT 0,
    red7f           SMALLINT NOT NULL DEFAULT 0,
    grn7f           SMALLINT NOT NULL DEFAULT 0,
    blu7f           SMALLINT NOT NULL DEFAULT 0,
    red8x           SMALLINT NOT NULL DEFAULT 0,
    grn8x           SMALLINT NOT NULL DEFAULT 0,
    blu8x           SMALLINT NOT NULL DEFAULT 0,
    red9x           SMALLINT NOT NULL DEFAULT 0,
    grn9x           SMALLINT NOT NULL DEFAULT 0,
    blu9x           SMALLINT NOT NULL DEFAULT 0
);

-- NPC emote text (combat shouts, idle chatter)
-- EQEmu: npc_emotes. Unchanged.
CREATE TABLE IF NOT EXISTS npc_emotes (
    id       SERIAL PRIMARY KEY,
    emoteid  INTEGER NOT NULL DEFAULT 0,
    event_   SMALLINT NOT NULL DEFAULT 0,
    type     SMALLINT NOT NULL DEFAULT 0,
    text     TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_npc_emotes_emoteid ON npc_emotes(emoteid);

-- NPC faction primary records
-- EQEmu: npc_faction. Links NPC types to faction behavior.
CREATE TABLE IF NOT EXISTS npc_faction (
    id             INTEGER PRIMARY KEY,
    name           TEXT NOT NULL DEFAULT '',
    primaryfaction INTEGER NOT NULL DEFAULT 0,
    ignore_primary_assist SMALLINT NOT NULL DEFAULT 0
);

-- NPC faction entries (how an NPC reacts to other factions)
-- EQEmu: npc_faction_entries. Unchanged.
CREATE TABLE IF NOT EXISTS npc_faction_entries (
    npc_faction_id INTEGER NOT NULL DEFAULT 0,
    faction_id     INTEGER NOT NULL DEFAULT 0,
    value          INTEGER NOT NULL DEFAULT 0,
    npc_value      SMALLINT NOT NULL DEFAULT 0,
    temp           SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (npc_faction_id, faction_id)
);

-- NPC spell effect sets (passive effects on NPCs)
-- EQEmu: npc_spells_effects. Unchanged.
CREATE TABLE IF NOT EXISTS npc_spells_effects (
    id       INTEGER PRIMARY KEY,
    name     TEXT NOT NULL DEFAULT '',
    parent_list INTEGER NOT NULL DEFAULT 0
);

-- Individual entries in an NPC spell effect set
-- EQEmu: npc_spells_effects_entries. Unchanged.
CREATE TABLE IF NOT EXISTS npc_spells_effects_entries (
    id                  SERIAL PRIMARY KEY,
    npc_spells_effects_id INTEGER NOT NULL DEFAULT 0,
    spell_effect_id     SMALLINT NOT NULL DEFAULT 0,
    minlevel            SMALLINT NOT NULL DEFAULT 0,
    maxlevel            SMALLINT NOT NULL DEFAULT 0,
    se_base             INTEGER NOT NULL DEFAULT 0,
    se_limit            INTEGER NOT NULL DEFAULT 0,
    se_max              INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_npc_spell_effects_set ON npc_spells_effects_entries(npc_spells_effects_id);

-- Master faction definitions
-- EQEmu: faction_list. Unchanged.
CREATE TABLE IF NOT EXISTS faction_list (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL DEFAULT '',
    base INTEGER NOT NULL DEFAULT 0
);

-- Faction standing modifiers (race/class/deity adjustments)
-- EQEmu: faction_list_mod. Unchanged.
CREATE TABLE IF NOT EXISTS faction_list_mod (
    id        SERIAL PRIMARY KEY,
    faction_id INTEGER NOT NULL DEFAULT 0,
    mod_name  TEXT NOT NULL DEFAULT '',
    mod       INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_faction_list_mod_faction ON faction_list_mod(faction_id);

-- Damage shield type definitions
-- EQEmu: damageshieldtypes. Maps spell effect IDs to damage types.
CREATE TABLE IF NOT EXISTS damageshieldtypes (
    spellid       INTEGER PRIMARY KEY,
    type          SMALLINT NOT NULL DEFAULT 0
);

-- Global spell overrides
-- EQEmu: spell_globals. Allows per-zone spell behavior changes.
CREATE TABLE IF NOT EXISTS spell_globals (
    spellid   INTEGER PRIMARY KEY,
    spell_name TEXT NOT NULL DEFAULT '',
    qglobal   TEXT NOT NULL DEFAULT '',
    value     TEXT NOT NULL DEFAULT ''
);
