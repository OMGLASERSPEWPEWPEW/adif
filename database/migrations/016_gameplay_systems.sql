-- 016_gameplay_systems.sql
-- Pets, quests, fishing, foraging, tradeskills, bazaar.
--
-- EQEmu: pets, quest_globals, fishing, forage, tradeskill_recipe,
-- tradeskill_recipe_entries, trader.
-- ADIF: Identical — core gameplay systems.

-- Pet definitions (which NPC a pet spell summons)
-- EQEmu: pets. Unchanged.
CREATE TABLE IF NOT EXISTS pets (
    type         TEXT NOT NULL DEFAULT '',
    petpower     INTEGER NOT NULL DEFAULT 0,
    npcID        INTEGER NOT NULL DEFAULT 0,
    temp         SMALLINT NOT NULL DEFAULT 0,
    petcontrol   SMALLINT NOT NULL DEFAULT 0,
    petnaming    SMALLINT NOT NULL DEFAULT 0,
    monsterflag  SMALLINT NOT NULL DEFAULT 0,
    equipmentset INTEGER NOT NULL DEFAULT -1,
    PRIMARY KEY (type, petpower)
);

-- Quest global variables (persistent across zones/sessions)
-- EQEmu: quest_globals. Used by Lua/Perl quest scripts.
CREATE TABLE IF NOT EXISTS quest_globals (
    charid  INTEGER NOT NULL DEFAULT 0,
    npcid   INTEGER NOT NULL DEFAULT 0,
    zoneid  INTEGER NOT NULL DEFAULT 0,
    name    TEXT NOT NULL DEFAULT '',
    value   TEXT NOT NULL DEFAULT '?',
    expdate INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (charid, npcid, zoneid, name)
);

-- Fishing loot tables per zone
-- EQEmu: fishing. Unchanged.
CREATE TABLE IF NOT EXISTS fishing (
    id                      SERIAL PRIMARY KEY,
    zoneid                  INTEGER NOT NULL DEFAULT 0,
    itemid                  INTEGER NOT NULL DEFAULT 0,
    skill_level             SMALLINT NOT NULL DEFAULT 0,
    chance                  SMALLINT NOT NULL DEFAULT 0,
    npc_id                  INTEGER NOT NULL DEFAULT 0,
    npc_chance              INTEGER NOT NULL DEFAULT 0,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);

-- Foraging loot tables per zone
-- EQEmu: forage. Unchanged.
CREATE TABLE IF NOT EXISTS forage (
    id                      SERIAL PRIMARY KEY,
    zoneid                  INTEGER NOT NULL DEFAULT 0,
    itemid                  INTEGER NOT NULL DEFAULT 0,
    level                   SMALLINT NOT NULL DEFAULT 0,
    chance                  SMALLINT NOT NULL DEFAULT 0,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);

-- Tradeskill recipe definitions
-- EQEmu: tradeskill_recipe. Unchanged.
CREATE TABLE IF NOT EXISTS tradeskill_recipe (
    id                      SERIAL PRIMARY KEY,
    name                    TEXT NOT NULL DEFAULT '',
    tradeskill              SMALLINT NOT NULL DEFAULT 0,
    skillneeded             SMALLINT NOT NULL DEFAULT 0,
    trivial                 SMALLINT NOT NULL DEFAULT 0,
    nofail                  BOOLEAN NOT NULL DEFAULT FALSE,
    replace_container       BOOLEAN NOT NULL DEFAULT FALSE,
    notes                   TEXT NOT NULL DEFAULT '',
    must_learn              SMALLINT NOT NULL DEFAULT 0,
    learned_by_item_id      INTEGER NOT NULL DEFAULT 0,
    quest                   BOOLEAN NOT NULL DEFAULT FALSE,
    enabled                 BOOLEAN NOT NULL DEFAULT TRUE,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);

-- Tradeskill recipe components and results
-- EQEmu: tradeskill_recipe_entries. Unchanged.
CREATE TABLE IF NOT EXISTS tradeskill_recipe_entries (
    id             SERIAL PRIMARY KEY,
    recipe_id      INTEGER NOT NULL DEFAULT 0 REFERENCES tradeskill_recipe(id),
    item_id        INTEGER NOT NULL DEFAULT 0,
    successcount   SMALLINT NOT NULL DEFAULT 0,
    failcount      SMALLINT NOT NULL DEFAULT 0,
    componentcount SMALLINT NOT NULL DEFAULT 1,
    iscontainer    BOOLEAN NOT NULL DEFAULT FALSE,
    isnoreplace    BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX IF NOT EXISTS idx_recipe_entries_recipe ON tradeskill_recipe_entries(recipe_id);

-- Bazaar trader listings (player shops)
-- EQEmu: trader. Unchanged.
CREATE TABLE IF NOT EXISTS trader (
    char_id  INTEGER NOT NULL DEFAULT 0,
    item_id  INTEGER NOT NULL DEFAULT 0,
    serialnumber INTEGER NOT NULL DEFAULT 0,
    charges  SMALLINT NOT NULL DEFAULT 0,
    item_cost INTEGER NOT NULL DEFAULT 0,
    slot_id  SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id, slot_id)
);
