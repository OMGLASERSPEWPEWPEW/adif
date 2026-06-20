-- Player characters.
-- EQEmu: character_data (62 columns) + 5 satellite tables.
-- ADIF: one core table + JSONB for appearance, separate tables for skills/spells/inventory.

CREATE TABLE characters (
    id              SERIAL PRIMARY KEY,
    account_id      INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name            VARCHAR(64) UNIQUE NOT NULL,
    last_name       VARCHAR(64) NOT NULL DEFAULT '',
    title           VARCHAR(64) NOT NULL DEFAULT '',

    -- Class/Race
    race            SMALLINT NOT NULL,
    class_id        SMALLINT NOT NULL,
    level           SMALLINT NOT NULL DEFAULT 1,
    gender          SMALLINT NOT NULL DEFAULT 0,
    deity           SMALLINT NOT NULL DEFAULT 0,

    -- Attributes
    str             SMALLINT NOT NULL DEFAULT 75,
    sta             SMALLINT NOT NULL DEFAULT 75,
    dex             SMALLINT NOT NULL DEFAULT 75,
    agi             SMALLINT NOT NULL DEFAULT 75,
    int_            SMALLINT NOT NULL DEFAULT 75,
    wis             SMALLINT NOT NULL DEFAULT 75,
    cha             SMALLINT NOT NULL DEFAULT 75,

    -- Vitals
    current_hp      INTEGER NOT NULL DEFAULT 100,
    max_hp          INTEGER NOT NULL DEFAULT 100,
    current_mana    INTEGER NOT NULL DEFAULT 0,
    max_mana        INTEGER NOT NULL DEFAULT 0,
    current_endurance INTEGER NOT NULL DEFAULT 100,
    max_endurance   INTEGER NOT NULL DEFAULT 100,

    -- Experience
    experience      BIGINT NOT NULL DEFAULT 0,
    aa_experience   BIGINT NOT NULL DEFAULT 0,
    aa_points_spent INTEGER NOT NULL DEFAULT 0,
    aa_points_unspent INTEGER NOT NULL DEFAULT 0,

    -- Position
    zone_id         INTEGER NOT NULL REFERENCES zones(id),
    x               REAL NOT NULL DEFAULT 0,
    y               REAL NOT NULL DEFAULT 0,
    z               REAL NOT NULL DEFAULT 0,
    heading         REAL NOT NULL DEFAULT 0,

    -- Currency
    platinum        INTEGER NOT NULL DEFAULT 0,
    gold            INTEGER NOT NULL DEFAULT 0,
    silver          INTEGER NOT NULL DEFAULT 0,
    copper          INTEGER NOT NULL DEFAULT 0,
    bank_platinum   INTEGER NOT NULL DEFAULT 0,
    bank_gold       INTEGER NOT NULL DEFAULT 0,
    bank_silver     INTEGER NOT NULL DEFAULT 0,
    bank_copper     INTEGER NOT NULL DEFAULT 0,

    -- Appearance (JSONB replaces EQEmu's 7 separate columns)
    appearance      JSONB NOT NULL DEFAULT '{}',

    -- Bind points (JSONB array of up to 5 locations)
    bind_points     JSONB NOT NULL DEFAULT '[]',

    -- Flags
    is_gm           BOOLEAN NOT NULL DEFAULT FALSE,
    pvp_enabled     BOOLEAN NOT NULL DEFAULT FALSE,
    anonymous       BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,

    -- Guild
    guild_id        INTEGER,
    guild_rank      SMALLINT NOT NULL DEFAULT 0,

    -- Timestamps
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at   TIMESTAMPTZ,
    time_played     INTEGER NOT NULL DEFAULT 0  -- seconds
);

CREATE INDEX idx_characters_account ON characters (account_id);
CREATE INDEX idx_characters_name ON characters (name);
CREATE INDEX idx_characters_zone ON characters (zone_id);

-- Character skills (index = skill ID, value = skill level).
CREATE TABLE character_skills (
    character_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    skill_id        SMALLINT NOT NULL,
    value           SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (character_id, skill_id)
);

-- Character spell book.
CREATE TABLE character_spells (
    character_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    slot            SMALLINT NOT NULL,
    spell_id        INTEGER NOT NULL,
    PRIMARY KEY (character_id, slot)
);

-- Character memorized spell bar (8 slots).
CREATE TABLE character_memmed_spells (
    character_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    slot            SMALLINT NOT NULL CHECK (slot >= 0 AND slot < 8),
    spell_id        INTEGER NOT NULL,
    PRIMARY KEY (character_id, slot)
);

-- Character inventory.
CREATE TABLE character_inventory (
    character_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    slot_id         SMALLINT NOT NULL,
    item_id         INTEGER NOT NULL,
    charges         SMALLINT NOT NULL DEFAULT 0,
    quantity        SMALLINT NOT NULL DEFAULT 1,
    PRIMARY KEY (character_id, slot_id)
);

-- Active buffs (persisted across zone/logout).
CREATE TABLE character_buffs (
    character_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    slot            SMALLINT NOT NULL,
    spell_id        INTEGER NOT NULL,
    caster_level    SMALLINT NOT NULL DEFAULT 0,
    duration_remaining INTEGER NOT NULL DEFAULT 0,
    counters        INTEGER NOT NULL DEFAULT 0,
    caster_id       INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (character_id, slot)
);
