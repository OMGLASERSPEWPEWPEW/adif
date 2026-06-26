-- 039_missing_character_tables.sql
-- Create ~22 missing tables that produce "relation does not exist" errors
-- during zone boot and character load. Schemas from akk-stack MariaDB.
--
-- EQEmu: These are all standard EQEmu tables. ADIF creates them with
-- identical schemas to allow the C++ server code to function.

BEGIN;

-- character_item_recast: tracks item reuse timers
CREATE TABLE IF NOT EXISTS character_item_recast (
    id              INTEGER NOT NULL DEFAULT 0,
    recast_type     INTEGER NOT NULL DEFAULT 0,
    timestamp       INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id, recast_type)
);
CREATE INDEX IF NOT EXISTS idx_character_item_recast_id ON character_item_recast (id);

-- character_evolving_items: tracks evolving item progress
CREATE TABLE IF NOT EXISTS character_evolving_items (
    id              SERIAL PRIMARY KEY,
    character_id    INTEGER DEFAULT 0,
    item_id         INTEGER DEFAULT 0,
    activated       SMALLINT DEFAULT 0,
    equipped        SMALLINT DEFAULT 0,
    current_amount  BIGINT DEFAULT 0,
    progression     DOUBLE PRECISION DEFAULT 0,
    final_item_id   INTEGER DEFAULT 0,
    deleted_at      TIMESTAMP DEFAULT NULL
);

-- sharedbank: cross-character shared bank slots
CREATE TABLE IF NOT EXISTS sharedbank (
    account_id          INTEGER NOT NULL DEFAULT 0,
    slot_id             INTEGER NOT NULL DEFAULT 0,
    item_id             INTEGER NOT NULL DEFAULT 0,
    charges             SMALLINT NOT NULL DEFAULT 0,
    color               BIGINT NOT NULL DEFAULT 0,
    augment_one         INTEGER NOT NULL DEFAULT 0,
    augment_two         INTEGER NOT NULL DEFAULT 0,
    augment_three       INTEGER NOT NULL DEFAULT 0,
    augment_four        INTEGER NOT NULL DEFAULT 0,
    augment_five        INTEGER NOT NULL DEFAULT 0,
    augment_six         INTEGER NOT NULL DEFAULT 0,
    custom_data         TEXT DEFAULT NULL,
    ornament_icon       INTEGER NOT NULL DEFAULT 0,
    ornament_idfile     INTEGER NOT NULL DEFAULT 0,
    ornament_hero_model INTEGER NOT NULL DEFAULT 0,
    guid                BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (account_id, slot_id)
);

-- character_bandolier: saved weapon sets
CREATE TABLE IF NOT EXISTS character_bandolier (
    id              INTEGER NOT NULL DEFAULT 0,
    bandolier_id    SMALLINT NOT NULL DEFAULT 0,
    bandolier_slot  SMALLINT NOT NULL DEFAULT 0,
    item_id         INTEGER NOT NULL DEFAULT 0,
    icon            INTEGER NOT NULL DEFAULT 0,
    bandolier_name  VARCHAR(32) NOT NULL DEFAULT '0',
    PRIMARY KEY (id, bandolier_id, bandolier_slot)
);
CREATE INDEX IF NOT EXISTS idx_character_bandolier_id ON character_bandolier (id);

-- character_potionbelt: potion belt slots
CREATE TABLE IF NOT EXISTS character_potionbelt (
    id          INTEGER NOT NULL DEFAULT 0,
    potion_id   SMALLINT NOT NULL DEFAULT 0,
    item_id     INTEGER NOT NULL DEFAULT 0,
    icon        INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id, potion_id)
);
CREATE INDEX IF NOT EXISTS idx_character_potionbelt_id ON character_potionbelt (id);

-- character_leadership_abilities: group/raid leadership AAs
CREATE TABLE IF NOT EXISTS character_leadership_abilities (
    id      INTEGER NOT NULL DEFAULT 0,
    slot    SMALLINT NOT NULL DEFAULT 0,
    "rank"  SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id, slot)
);
CREATE INDEX IF NOT EXISTS idx_character_leadership_id ON character_leadership_abilities (id);

-- character_tribute: tribute settings per character
CREATE TABLE IF NOT EXISTS character_tribute (
    id              SERIAL PRIMARY KEY,
    character_id    INTEGER NOT NULL DEFAULT 0,
    tier            SMALLINT NOT NULL DEFAULT 0,
    tribute         INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_character_tribute_charid ON character_tribute (character_id);

-- character_exp_modifiers: per-zone XP modifiers
CREATE TABLE IF NOT EXISTS character_exp_modifiers (
    character_id        INTEGER NOT NULL,
    zone_id             INTEGER NOT NULL,
    instance_version    INTEGER NOT NULL DEFAULT -1,
    aa_modifier         REAL NOT NULL DEFAULT 1,
    exp_modifier        REAL NOT NULL DEFAULT 1,
    PRIMARY KEY (character_id, zone_id, instance_version)
);

-- character_tasks: active task assignments
CREATE TABLE IF NOT EXISTS character_tasks (
    charid          INTEGER NOT NULL DEFAULT 0,
    taskid          INTEGER NOT NULL DEFAULT 0,
    slot            INTEGER NOT NULL DEFAULT 0,
    type            SMALLINT NOT NULL DEFAULT 0,
    acceptedtime    INTEGER DEFAULT NULL,
    was_rewarded    SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (charid, taskid)
);

-- character_activities: task activity progress
CREATE TABLE IF NOT EXISTS character_activities (
    charid      INTEGER NOT NULL DEFAULT 0,
    taskid      INTEGER NOT NULL DEFAULT 0,
    activityid  INTEGER NOT NULL DEFAULT 0,
    donecount   INTEGER NOT NULL DEFAULT 0,
    completed   SMALLINT DEFAULT 0,
    PRIMARY KEY (charid, taskid, activityid)
);

-- completed_tasks: task completion history
CREATE TABLE IF NOT EXISTS completed_tasks (
    charid          INTEGER NOT NULL DEFAULT 0,
    completedtime   INTEGER NOT NULL DEFAULT 0,
    taskid          INTEGER NOT NULL DEFAULT 0,
    activityid      INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (charid, completedtime, taskid, activityid)
);

-- character_enabledtasks: unlocked tasks
CREATE TABLE IF NOT EXISTS character_enabledtasks (
    charid  INTEGER NOT NULL DEFAULT 0,
    taskid  INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (charid, taskid)
);

-- character_disciplines: learned combat disciplines
CREATE TABLE IF NOT EXISTS character_disciplines (
    id      INTEGER NOT NULL DEFAULT 0,
    slot_id SMALLINT NOT NULL DEFAULT 0,
    disc_id SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (id, slot_id)
);
CREATE INDEX IF NOT EXISTS idx_character_disciplines_id ON character_disciplines (id);

-- character_auras: active aura spells
CREATE TABLE IF NOT EXISTS character_auras (
    id          INTEGER NOT NULL,
    slot        SMALLINT NOT NULL,
    spell_id    INTEGER NOT NULL,
    PRIMARY KEY (id, slot)
);

-- character_alt_currency: alternate currency balances
CREATE TABLE IF NOT EXISTS character_alt_currency (
    char_id     INTEGER NOT NULL,
    currency_id INTEGER NOT NULL,
    amount      INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id, currency_id)
);

-- character_instance_safereturns: instance exit coordinates
CREATE TABLE IF NOT EXISTS character_instance_safereturns (
    id              SERIAL PRIMARY KEY,
    character_id    INTEGER NOT NULL DEFAULT 0,
    instance_zone_id INTEGER NOT NULL DEFAULT 0,
    instance_id     INTEGER NOT NULL DEFAULT 0,
    safe_zone_id    INTEGER NOT NULL DEFAULT 0,
    safe_x          REAL NOT NULL DEFAULT 0,
    safe_y          REAL NOT NULL DEFAULT 0,
    safe_z          REAL NOT NULL DEFAULT 0,
    safe_heading    REAL NOT NULL DEFAULT 0,
    UNIQUE (character_id)
);

-- character_peqzone_flags: PEQ zone access flags
CREATE TABLE IF NOT EXISTS character_peqzone_flags (
    id      INTEGER NOT NULL DEFAULT 0,
    zone_id INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id, zone_id)
);

-- completed_shared_task_members: shared task completion tracking
CREATE TABLE IF NOT EXISTS completed_shared_task_members (
    shared_task_id  BIGINT NOT NULL,
    character_id    BIGINT NOT NULL,
    is_leader       SMALLINT DEFAULT NULL,
    PRIMARY KEY (shared_task_id, character_id)
);

-- adventure_members: LDoN adventure group members
CREATE TABLE IF NOT EXISTS adventure_members (
    id      INTEGER NOT NULL,
    charid  INTEGER NOT NULL,
    PRIMARY KEY (charid)
);
CREATE INDEX IF NOT EXISTS idx_adventure_members_id ON adventure_members (id);

-- keyring: collected key items
CREATE TABLE IF NOT EXISTS keyring (
    id      SERIAL PRIMARY KEY,
    char_id INTEGER NOT NULL DEFAULT 0,
    item_id INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_keyring_charid_itemid ON keyring (char_id, item_id);

-- veteran_reward_templates: veteran claim rewards
CREATE TABLE IF NOT EXISTS veteran_reward_templates (
    claim_id    INTEGER NOT NULL,
    name        VARCHAR(64) NOT NULL DEFAULT '',
    item_id     INTEGER NOT NULL DEFAULT 0,
    charges     SMALLINT NOT NULL DEFAULT 0,
    reward_slot SMALLINT NOT NULL DEFAULT 0,
    UNIQUE (claim_id, reward_slot)
);
CREATE INDEX IF NOT EXISTS idx_veteran_reward_claim ON veteran_reward_templates (claim_id);

-- adventure_template_entry_flavor: LDoN adventure flavor text
CREATE TABLE IF NOT EXISTS adventure_template_entry_flavor (
    id      INTEGER PRIMARY KEY,
    text    TEXT NOT NULL DEFAULT ''
);

COMMIT;
