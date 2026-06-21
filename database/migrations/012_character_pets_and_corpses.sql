-- 012_character_pets_and_corpses.sql
-- Death and pet persistence systems.
--
-- EQEmu: character_corpses is one of the most complex tables (~40 columns).
-- ADIF: Kept flat here because corpse data is ephemeral and the C++ code
-- does heavy direct-column access. Not worth JSONB modernization.

-- Player corpses left in the world after death
-- EQEmu: character_corpses (~40 columns). Unchanged — ephemeral data.
CREATE TABLE IF NOT EXISTS character_corpses (
    id            SERIAL PRIMARY KEY,
    charid        INTEGER NOT NULL DEFAULT 0,
    charname      TEXT NOT NULL DEFAULT '',
    zone_id       INTEGER NOT NULL DEFAULT 0,
    x             REAL NOT NULL DEFAULT 0,
    y             REAL NOT NULL DEFAULT 0,
    z             REAL NOT NULL DEFAULT 0,
    heading       REAL NOT NULL DEFAULT 0,
    time_of_death TIMESTAMP NOT NULL DEFAULT NOW(),
    is_buried     BOOLEAN NOT NULL DEFAULT FALSE,
    is_rezzed     BOOLEAN NOT NULL DEFAULT FALSE,
    is_locked     BOOLEAN NOT NULL DEFAULT FALSE,
    exp           INTEGER NOT NULL DEFAULT 0,
    gmexp         INTEGER NOT NULL DEFAULT 0,
    size          REAL NOT NULL DEFAULT 0,
    level         SMALLINT NOT NULL DEFAULT 0,
    race          SMALLINT NOT NULL DEFAULT 0,
    gender        SMALLINT NOT NULL DEFAULT 0,
    "class"       SMALLINT NOT NULL DEFAULT 0,
    deity         SMALLINT NOT NULL DEFAULT 0,
    texture       SMALLINT NOT NULL DEFAULT 0,
    helm_texture  SMALLINT NOT NULL DEFAULT 0,
    copper        INTEGER NOT NULL DEFAULT 0,
    silver        INTEGER NOT NULL DEFAULT 0,
    gold          INTEGER NOT NULL DEFAULT 0,
    platinum      INTEGER NOT NULL DEFAULT 0,
    hair_color    SMALLINT NOT NULL DEFAULT 0,
    beard_color   SMALLINT NOT NULL DEFAULT 0,
    eye_color_1   SMALLINT NOT NULL DEFAULT 0,
    eye_color_2   SMALLINT NOT NULL DEFAULT 0,
    hair_style    SMALLINT NOT NULL DEFAULT 0,
    face          SMALLINT NOT NULL DEFAULT 0,
    beard         SMALLINT NOT NULL DEFAULT 0,
    wc_1          INTEGER NOT NULL DEFAULT 0,
    wc_2          INTEGER NOT NULL DEFAULT 0,
    wc_3          INTEGER NOT NULL DEFAULT 0,
    wc_4          INTEGER NOT NULL DEFAULT 0,
    wc_5          INTEGER NOT NULL DEFAULT 0,
    wc_6          INTEGER NOT NULL DEFAULT 0,
    wc_7          INTEGER NOT NULL DEFAULT 0,
    wc_8          INTEGER NOT NULL DEFAULT 0,
    wc_9          INTEGER NOT NULL DEFAULT 0,
    killedby      SMALLINT NOT NULL DEFAULT 0,
    rezzable      BOOLEAN NOT NULL DEFAULT FALSE,
    rez_time      INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_corpses_charid ON character_corpses(charid);
CREATE INDEX IF NOT EXISTS idx_corpses_zone ON character_corpses(zone_id);

-- Items on a player corpse
-- EQEmu: character_corpse_items. Unchanged.
CREATE TABLE IF NOT EXISTS character_corpse_items (
    corpse_id  INTEGER NOT NULL REFERENCES character_corpses(id) ON DELETE CASCADE,
    equip_slot SMALLINT NOT NULL DEFAULT 0,
    item_id    INTEGER NOT NULL DEFAULT 0,
    charges    SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (corpse_id, equip_slot)
);

-- Saved pet state (persists across zones)
-- EQEmu: character_pet_info. Unchanged.
CREATE TABLE IF NOT EXISTS character_pet_info (
    char_id  INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    pet      SMALLINT NOT NULL DEFAULT 0,
    petname  TEXT NOT NULL DEFAULT '',
    petpower INTEGER NOT NULL DEFAULT 0,
    spell_id INTEGER NOT NULL DEFAULT 0,
    hp       INTEGER NOT NULL DEFAULT 0,
    mana     INTEGER NOT NULL DEFAULT 0,
    size     REAL NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id, pet)
);

-- Buffs on a saved pet
-- EQEmu: character_pet_buffs. Unchanged.
CREATE TABLE IF NOT EXISTS character_pet_buffs (
    char_id       INTEGER NOT NULL,
    pet           SMALLINT NOT NULL DEFAULT 0,
    slot          SMALLINT NOT NULL DEFAULT 0,
    spell_id      INTEGER NOT NULL DEFAULT 0,
    caster_level  SMALLINT NOT NULL DEFAULT 0,
    castername    TEXT NOT NULL DEFAULT '',
    ticsremaining INTEGER NOT NULL DEFAULT 0,
    counters      INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id, pet, slot),
    FOREIGN KEY (char_id, pet) REFERENCES character_pet_info(char_id, pet) ON DELETE CASCADE
);

-- Items equipped on a saved pet
-- EQEmu: character_pet_inventory. Unchanged.
CREATE TABLE IF NOT EXISTS character_pet_inventory (
    char_id INTEGER NOT NULL,
    pet     SMALLINT NOT NULL DEFAULT 0,
    slot    SMALLINT NOT NULL DEFAULT 0,
    item_id INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id, pet, slot),
    FOREIGN KEY (char_id, pet) REFERENCES character_pet_info(char_id, pet) ON DELETE CASCADE
);
