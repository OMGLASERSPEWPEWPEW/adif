-- Spells.
-- EQEmu: spells_new (150+ columns with 12 effect slots x 3 values each).
-- ADIF: core columns + JSONB for effect slots and messages.

CREATE TABLE spells (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(64) NOT NULL,

    -- Casting
    cast_time_ms    INTEGER NOT NULL DEFAULT 0,
    recovery_ms     INTEGER NOT NULL DEFAULT 0,
    recast_ms       INTEGER NOT NULL DEFAULT 0,
    mana_cost       INTEGER NOT NULL DEFAULT 0,
    endurance_cost  INTEGER NOT NULL DEFAULT 0,

    -- Range
    range           REAL NOT NULL DEFAULT 0,
    aoe_range       REAL NOT NULL DEFAULT 0,

    -- Duration
    buff_duration   INTEGER NOT NULL DEFAULT 0,      -- ticks

    -- Targeting
    target_type     SMALLINT NOT NULL DEFAULT 0,     -- self, single, group, aoe, etc.
    resist_type     SMALLINT NOT NULL DEFAULT 0,     -- magic, fire, cold, etc.
    resist_diff     SMALLINT NOT NULL DEFAULT 0,

    -- Requirements
    classes         JSONB NOT NULL DEFAULT '{}',
    -- Example: {"1": 5, "2": 10} = warrior at level 5, cleric at level 10

    -- Components/reagents
    components      JSONB NOT NULL DEFAULT '[]',
    -- Example: [{"item_id": 123, "count": 1}]

    -- Effect slots (JSONB replaces 36 separate columns)
    effects         JSONB NOT NULL DEFAULT '[]',
    -- Example: [{"id": 0, "base": -50, "limit": 0, "max": 0}, ...]

    -- Messages
    messages        JSONB NOT NULL DEFAULT '{}',
    -- Example: {"you_cast": "You cast...", "on_target": "feels better", "fades": "The spell fades."}

    -- Icons
    icon            INTEGER NOT NULL DEFAULT 0,
    mem_icon        INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_spells_name ON spells (name);

-- NPC spell lists.
CREATE TABLE npc_spell_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(128) NOT NULL DEFAULT ''
);

CREATE TABLE npc_spell_entries (
    spell_list_id   INTEGER NOT NULL REFERENCES npc_spell_lists(id) ON DELETE CASCADE,
    spell_id        INTEGER NOT NULL REFERENCES spells(id) ON DELETE CASCADE,
    priority        SMALLINT NOT NULL DEFAULT 0,
    min_hp_pct      SMALLINT NOT NULL DEFAULT 0,
    max_hp_pct      SMALLINT NOT NULL DEFAULT 100,
    PRIMARY KEY (spell_list_id, spell_id)
);
