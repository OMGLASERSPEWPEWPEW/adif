-- Migration 043: Create all remaining tables for 100% MariaDB parity
-- Generated from akk-stack MariaDB (peq) ground truth, 2026-06-26
-- Creates 48 tables that exist in MariaDB but not PostgreSQL

BEGIN;

-- ============================================================
-- 1. adventure_details
-- EQEmu: adventure_details (9 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS adventure_details (
    id                SERIAL PRIMARY KEY,
    adventure_id      SMALLINT    NOT NULL DEFAULT 0,
    instance_id       INTEGER     NOT NULL DEFAULT -1,
    "count"           SMALLINT    NOT NULL DEFAULT 0,
    assassinate_count SMALLINT    NOT NULL DEFAULT 0,
    status            SMALLINT    NOT NULL DEFAULT 0,
    time_created      INTEGER     NOT NULL DEFAULT 0,
    time_zoned        INTEGER     NOT NULL DEFAULT 0,
    time_completed    INTEGER     NOT NULL DEFAULT 0
);

-- ============================================================
-- 2. auras
-- EQEmu: auras (11 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS auras (
    "type"     INTEGER     NOT NULL,
    npc_type   INTEGER     NOT NULL,
    "name"     VARCHAR(64) NOT NULL,
    spell_id   INTEGER     NOT NULL,
    distance   INTEGER     NOT NULL DEFAULT 60,
    aura_type  INTEGER     NOT NULL DEFAULT 1,
    spawn_type INTEGER     NOT NULL DEFAULT 0,
    movement   INTEGER     NOT NULL DEFAULT 0,
    duration   INTEGER     NOT NULL DEFAULT 5400,
    icon       INTEGER     NOT NULL DEFAULT -1,
    cast_time  INTEGER     NOT NULL DEFAULT 0,
    PRIMARY KEY ("type")
);

-- ============================================================
-- 3. books
-- EQEmu: books (4 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS books (
    id       SERIAL PRIMARY KEY,
    "name"   VARCHAR(30) NOT NULL DEFAULT '',
    txtfile  TEXT        NOT NULL,
    language INTEGER     NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_books_filename ON books ("name");

-- ============================================================
-- 4. bug_reports
-- EQEmu: bug_reports (32 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS bug_reports (
    id                  SERIAL PRIMARY KEY,
    "zone"              VARCHAR(32)   NOT NULL DEFAULT 'Unknown',
    client_version_id   INTEGER       NOT NULL DEFAULT 0,
    client_version_name VARCHAR(24)   NOT NULL DEFAULT 'Unknown',
    account_id          INTEGER       NOT NULL DEFAULT 0,
    character_id        INTEGER       NOT NULL DEFAULT 0,
    character_name      VARCHAR(64)   NOT NULL DEFAULT 'Unknown',
    reporter_spoof      SMALLINT      NOT NULL DEFAULT 1,
    category_id         INTEGER       NOT NULL DEFAULT 0,
    category_name       VARCHAR(64)   NOT NULL DEFAULT 'Other',
    reporter_name       VARCHAR(64)   NOT NULL DEFAULT 'Unknown',
    ui_path             VARCHAR(128)  NOT NULL DEFAULT 'Unknown',
    pos_x               REAL          NOT NULL DEFAULT 0,
    pos_y               REAL          NOT NULL DEFAULT 0,
    pos_z               REAL          NOT NULL DEFAULT 0,
    heading             INTEGER       NOT NULL DEFAULT 0,
    time_played         INTEGER       NOT NULL DEFAULT 0,
    target_id           INTEGER       NOT NULL DEFAULT 0,
    target_name         VARCHAR(64)   NOT NULL DEFAULT 'Unknown',
    optional_info_mask  INTEGER       NOT NULL DEFAULT 0,
    _can_duplicate      SMALLINT      NOT NULL DEFAULT 0,
    _crash_bug          SMALLINT      NOT NULL DEFAULT 0,
    _target_info        SMALLINT      NOT NULL DEFAULT 0,
    _character_flags    SMALLINT      NOT NULL DEFAULT 0,
    _unknown_value      SMALLINT      NOT NULL DEFAULT 0,
    bug_report          VARCHAR(1024) NOT NULL DEFAULT '',
    system_info         VARCHAR(1024) NOT NULL DEFAULT '',
    report_datetime     TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    bug_status          SMALLINT      NOT NULL DEFAULT 0,
    last_review         TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_reviewer       VARCHAR(64)   NOT NULL DEFAULT 'None',
    reviewer_notes      VARCHAR(1024) NOT NULL DEFAULT ''
);

-- ============================================================
-- 5. char_recipe_list
-- EQEmu: char_recipe_list (3 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS char_recipe_list (
    char_id   INTEGER NOT NULL,
    recipe_id INTEGER NOT NULL,
    madecount INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (char_id, recipe_id)
);

-- ============================================================
-- 6. character_parcels
-- EQEmu: character_parcels (15 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS character_parcels (
    id            SERIAL PRIMARY KEY,
    char_id       INTEGER       NOT NULL DEFAULT 0,
    item_id       INTEGER       NOT NULL DEFAULT 0,
    aug_slot_1    INTEGER       NOT NULL DEFAULT 0,
    aug_slot_2    INTEGER       NOT NULL DEFAULT 0,
    aug_slot_3    INTEGER       NOT NULL DEFAULT 0,
    aug_slot_4    INTEGER       NOT NULL DEFAULT 0,
    aug_slot_5    INTEGER       NOT NULL DEFAULT 0,
    aug_slot_6    INTEGER       NOT NULL DEFAULT 0,
    slot_id       INTEGER       NOT NULL DEFAULT 0,
    quantity      INTEGER       NOT NULL DEFAULT 0,
    evolve_amount INTEGER       NOT NULL DEFAULT 0,
    from_name     VARCHAR(64)   DEFAULT NULL,
    note          VARCHAR(1024) DEFAULT NULL,
    sent_date     TIMESTAMP     DEFAULT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_character_parcels_data_constraint
    ON character_parcels (slot_id, char_id);

-- ============================================================
-- 7. character_parcels_containers
-- EQEmu: character_parcels_containers (12 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS character_parcels_containers (
    id            SERIAL PRIMARY KEY,
    parcels_id    INTEGER NOT NULL DEFAULT 0,
    slot_id       INTEGER NOT NULL DEFAULT 0,
    item_id       INTEGER NOT NULL DEFAULT 0,
    aug_slot_1    INTEGER NOT NULL DEFAULT 0,
    aug_slot_2    INTEGER NOT NULL DEFAULT 0,
    aug_slot_3    INTEGER NOT NULL DEFAULT 0,
    aug_slot_4    INTEGER NOT NULL DEFAULT 0,
    aug_slot_5    INTEGER NOT NULL DEFAULT 0,
    aug_slot_6    INTEGER NOT NULL DEFAULT 0,
    quantity      INTEGER NOT NULL DEFAULT 0,
    evolve_amount INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_character_parcels_containers_parcels_id
    ON character_parcels_containers (parcels_id);

-- ============================================================
-- 8. chatchannel_reserved_names
-- EQEmu: chatchannel_reserved_names (2 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS chatchannel_reserved_names (
    id   SERIAL PRIMARY KEY,
    "name" VARCHAR(64) NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_chatchannel_reserved_names_name
    ON chatchannel_reserved_names ("name");

-- ============================================================
-- 9. chatchannels
-- EQEmu: chatchannels (5 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS chatchannels (
    id        SERIAL PRIMARY KEY,
    "name"    VARCHAR(64) NOT NULL DEFAULT '',
    owner     VARCHAR(64) NOT NULL DEFAULT '',
    password  VARCHAR(64) NOT NULL DEFAULT '',
    minstatus INTEGER     NOT NULL DEFAULT 0
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_chatchannels_name ON chatchannels ("name");

-- ============================================================
-- 10. completed_shared_task_activity_state
-- EQEmu: completed_shared_task_activity_state (5 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS completed_shared_task_activity_state (
    shared_task_id BIGINT    NOT NULL,
    activity_id    INTEGER   NOT NULL,
    done_count     INTEGER   DEFAULT NULL,
    updated_time   TIMESTAMP DEFAULT NULL,
    completed_time TIMESTAMP DEFAULT NULL,
    PRIMARY KEY (shared_task_id, activity_id)
);

-- ============================================================
-- 11. db_str
-- EQEmu: db_str (3 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS db_str (
    id    INTEGER NOT NULL,
    "type"  INTEGER NOT NULL,
    "value" TEXT    NOT NULL,
    PRIMARY KEY (id, "type")
);

-- ============================================================
-- 12. guild_bank
-- EQEmu: guild_bank (15 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS guild_bank (
    id               SERIAL PRIMARY KEY,
    guild_id         INTEGER     NOT NULL DEFAULT 0,
    area             SMALLINT    NOT NULL DEFAULT 0,
    "slot"           INTEGER     NOT NULL DEFAULT 0,
    item_id          INTEGER     NOT NULL DEFAULT 0,
    augment_one_id   INTEGER     DEFAULT 0,
    augment_two_id   INTEGER     DEFAULT 0,
    augment_three_id INTEGER     DEFAULT 0,
    augment_four_id  INTEGER     DEFAULT 0,
    augment_five_id  INTEGER     DEFAULT 0,
    augment_six_id   INTEGER     DEFAULT 0,
    quantity         INTEGER     NOT NULL DEFAULT 0,
    donator          VARCHAR(64) DEFAULT NULL,
    permissions      SMALLINT    NOT NULL DEFAULT 0,
    who_for          VARCHAR(64) DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_guild_bank_area ON guild_bank (area);
CREATE INDEX IF NOT EXISTS idx_guild_bank_slot ON guild_bank ("slot");
CREATE INDEX IF NOT EXISTS idx_guild_bank_guild_id ON guild_bank (guild_id);

-- ============================================================
-- 13. guild_relations
-- EQEmu: guild_relations (3 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS guild_relations (
    guild1   INTEGER  NOT NULL DEFAULT 0,
    guild2   INTEGER  NOT NULL DEFAULT 0,
    relation SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (guild1, guild2)
);

-- ============================================================
-- 14. horses
-- EQEmu: horses (8 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS horses (
    id          SERIAL PRIMARY KEY,
    filename    VARCHAR(32)  NOT NULL DEFAULT '',
    race        SMALLINT     NOT NULL DEFAULT 216,
    gender      SMALLINT     NOT NULL DEFAULT 0,
    texture     SMALLINT     NOT NULL DEFAULT 0,
    helmtexture SMALLINT     NOT NULL DEFAULT -1,
    mountspeed  REAL         NOT NULL DEFAULT 0.75,
    notes       VARCHAR(64)  DEFAULT 'Notes'
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_horses_filename ON horses (filename);

-- ============================================================
-- 15. inventory_versions
-- EQEmu: inventory_versions (3 columns). Created for 100% MariaDB parity.
-- No primary key in MariaDB.
-- ============================================================
CREATE TABLE IF NOT EXISTS inventory_versions (
    version  INTEGER NOT NULL DEFAULT 0,
    step     INTEGER NOT NULL DEFAULT 0,
    bot_step INTEGER NOT NULL DEFAULT 0
);

-- ============================================================
-- 16. ip_exemptions
-- EQEmu: ip_exemptions (3 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS ip_exemptions (
    exemption_id     SERIAL PRIMARY KEY,
    exemption_ip     VARCHAR(255) DEFAULT NULL,
    exemption_amount INTEGER      DEFAULT NULL
);

-- ============================================================
-- 17. launcher_zones
-- EQEmu: launcher_zones (3 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS launcher_zones (
    launcher VARCHAR(64) NOT NULL DEFAULT '',
    "zone"   VARCHAR(32) NOT NULL DEFAULT '',
    port     INTEGER     NOT NULL DEFAULT 0,
    PRIMARY KEY (launcher, "zone")
);

-- ============================================================
-- 18. lfguild
-- EQEmu: lfguild (9 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS lfguild (
    "type"     SMALLINT     NOT NULL DEFAULT 0,
    "name"     VARCHAR(32)  NOT NULL,
    "comment"  VARCHAR(256) NOT NULL,
    fromlevel  SMALLINT     NOT NULL DEFAULT 0,
    tolevel    SMALLINT     NOT NULL DEFAULT 0,
    classes    INTEGER      NOT NULL DEFAULT 0,
    aacount    INTEGER      NOT NULL DEFAULT 0,
    timezone   INTEGER      NOT NULL DEFAULT 0,
    timeposted INTEGER      NOT NULL DEFAULT 0,
    PRIMARY KEY ("type", "name")
);

-- ============================================================
-- 19. mail
-- EQEmu: mail (8 columns). Created for 100% MariaDB parity.
-- IMPORTANT: "from" and "to" are PostgreSQL reserved words, must be quoted.
-- ============================================================
CREATE TABLE IF NOT EXISTS mail (
    msgid      SERIAL PRIMARY KEY,
    charid     INTEGER      NOT NULL DEFAULT 0,
    "timestamp" INTEGER     NOT NULL DEFAULT 0,
    "from"     VARCHAR(100) NOT NULL DEFAULT '',
    subject    VARCHAR(200) NOT NULL DEFAULT '',
    body       TEXT         NOT NULL,
    "to"       TEXT         NOT NULL,
    status     SMALLINT     NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_mail_charid ON mail (charid);

-- ============================================================
-- 20. merc_buffs
-- EQEmu: merc_buffs (19 columns). Created for 100% MariaDB parity.
-- Note: lowercase column names for PostgreSQL cleanliness.
-- ============================================================
CREATE TABLE IF NOT EXISTS merc_buffs (
    mercbuffid          SERIAL PRIMARY KEY,
    mercid              INTEGER  NOT NULL DEFAULT 0,
    spellid             INTEGER  NOT NULL DEFAULT 0,
    casterlevel         INTEGER  NOT NULL DEFAULT 0,
    durationformula     INTEGER  NOT NULL DEFAULT 0,
    ticsremaining       INTEGER  NOT NULL DEFAULT 0,
    poisoncounters      INTEGER  NOT NULL DEFAULT 0,
    diseasecounters     INTEGER  NOT NULL DEFAULT 0,
    cursecounters       INTEGER  NOT NULL DEFAULT 0,
    corruptioncounters  INTEGER  NOT NULL DEFAULT 0,
    hitcount            INTEGER  NOT NULL DEFAULT 0,
    meleerune           INTEGER  NOT NULL DEFAULT 0,
    magicrune           INTEGER  NOT NULL DEFAULT 0,
    dot_rune            INTEGER  NOT NULL DEFAULT 0,
    caston_x            INTEGER  NOT NULL DEFAULT 0,
    persistent          SMALLINT NOT NULL DEFAULT 0,
    caston_y            INTEGER  NOT NULL DEFAULT 0,
    caston_z            INTEGER  NOT NULL DEFAULT 0,
    extradichance       INTEGER  NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_merc_buffs_mercid ON merc_buffs (mercid);

-- ============================================================
-- 21. mercs
-- EQEmu: mercs (24 columns). Created for 100% MariaDB parity.
-- Note: lowercase column names for PostgreSQL cleanliness.
-- ============================================================
CREATE TABLE IF NOT EXISTS mercs (
    mercid             SERIAL PRIMARY KEY,
    ownercharacterid   INTEGER     NOT NULL,
    "slot"             SMALLINT    NOT NULL DEFAULT 0,
    "name"             VARCHAR(64) NOT NULL,
    templateid         INTEGER     NOT NULL DEFAULT 0,
    suspendedtime      INTEGER     NOT NULL DEFAULT 0,
    issuspended        SMALLINT    NOT NULL DEFAULT 0,
    timerremaining     INTEGER     NOT NULL DEFAULT 0,
    gender             SMALLINT    NOT NULL DEFAULT 0,
    mercsize           REAL        NOT NULL DEFAULT 5,
    stanceid           SMALLINT    NOT NULL DEFAULT 0,
    hp                 INTEGER     NOT NULL DEFAULT 0,
    mana               INTEGER     NOT NULL DEFAULT 0,
    endurance          INTEGER     NOT NULL DEFAULT 0,
    face               INTEGER     NOT NULL DEFAULT 1,
    luclinhairstyle    INTEGER     NOT NULL DEFAULT 1,
    luclinhaircolor    INTEGER     NOT NULL DEFAULT 1,
    luclineyecolor     INTEGER     NOT NULL DEFAULT 1,
    luclineyecolor2    INTEGER     NOT NULL DEFAULT 1,
    luclinbeardcolor   INTEGER     NOT NULL DEFAULT 1,
    luclinbeard        INTEGER     NOT NULL DEFAULT 0,
    drakkinheritage    INTEGER     NOT NULL DEFAULT 0,
    drakkintattoo      INTEGER     NOT NULL DEFAULT 0,
    drakkindetails     INTEGER     NOT NULL DEFAULT 0
);

-- ============================================================
-- 22. peq_admin
-- EQEmu: peq_admin (4 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS peq_admin (
    id            SERIAL PRIMARY KEY,
    login         VARCHAR(30)  NOT NULL,
    password      VARCHAR(255) NOT NULL,
    administrator INTEGER      NOT NULL DEFAULT 0
);

-- ============================================================
-- 23. pets_beastlord_data
-- EQEmu: pets_beastlord_data (7 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS pets_beastlord_data (
    player_race   INTEGER  NOT NULL DEFAULT 1,
    pet_race      INTEGER  NOT NULL DEFAULT 42,
    texture       SMALLINT NOT NULL DEFAULT 0,
    helm_texture  SMALLINT NOT NULL DEFAULT 0,
    gender        SMALLINT NOT NULL DEFAULT 2,
    size_modifier REAL     DEFAULT 1,
    face          SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (player_race)
);

-- ============================================================
-- 24. pets_equipmentset
-- EQEmu: pets_equipmentset (3 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS pets_equipmentset (
    set_id     INTEGER     NOT NULL,
    setname    VARCHAR(30) NOT NULL DEFAULT '',
    nested_set INTEGER     NOT NULL DEFAULT -1,
    PRIMARY KEY (set_id)
);

-- ============================================================
-- 25. pets_equipmentset_entries
-- EQEmu: pets_equipmentset_entries (3 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS pets_equipmentset_entries (
    set_id  INTEGER NOT NULL,
    "slot"  INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    PRIMARY KEY (set_id, "slot")
);

-- ============================================================
-- 26. player_event_aa_purchase
-- EQEmu: player_event_aa_purchase (6 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_aa_purchase (
    id            BIGSERIAL PRIMARY KEY,
    aa_ability_id INTEGER   DEFAULT 0,
    cost          INTEGER   DEFAULT 0,
    previous_id   INTEGER   DEFAULT 0,
    next_id       INTEGER   DEFAULT 0,
    created_at    TIMESTAMP DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_aa_purchase_created_at
    ON player_event_aa_purchase (created_at);

-- ============================================================
-- 27. player_event_killed_named_npc
-- EQEmu: player_event_killed_named_npc (7 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_killed_named_npc (
    id                            BIGSERIAL PRIMARY KEY,
    npc_id                        INTEGER     DEFAULT 0,
    npc_name                      VARCHAR(64) DEFAULT NULL,
    combat_time_seconds           INTEGER     DEFAULT 0,
    total_damage_per_second_taken BIGINT      DEFAULT 0,
    total_heal_per_second_taken   BIGINT      DEFAULT 0,
    created_at                    TIMESTAMP   DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_killed_named_npc_npc_id
    ON player_event_killed_named_npc (npc_id);
CREATE INDEX IF NOT EXISTS idx_player_event_killed_named_npc_created_at
    ON player_event_killed_named_npc (created_at);

-- ============================================================
-- 28. player_event_killed_npc
-- EQEmu: player_event_killed_npc (7 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_killed_npc (
    id                            BIGSERIAL PRIMARY KEY,
    npc_id                        INTEGER     DEFAULT 0,
    npc_name                      VARCHAR(64) DEFAULT NULL,
    combat_time_seconds           INTEGER     DEFAULT 0,
    total_damage_per_second_taken BIGINT      DEFAULT 0,
    total_heal_per_second_taken   BIGINT      DEFAULT 0,
    created_at                    TIMESTAMP   DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_killed_npc_npc_id
    ON player_event_killed_npc (npc_id);
CREATE INDEX IF NOT EXISTS idx_player_event_killed_npc_created_at
    ON player_event_killed_npc (created_at);

-- ============================================================
-- 29. player_event_killed_raid_npc
-- EQEmu: player_event_killed_raid_npc (7 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_killed_raid_npc (
    id                            BIGSERIAL PRIMARY KEY,
    npc_id                        INTEGER     DEFAULT 0,
    npc_name                      VARCHAR(64) DEFAULT NULL,
    combat_time_seconds           INTEGER     DEFAULT 0,
    total_damage_per_second_taken BIGINT      DEFAULT 0,
    total_heal_per_second_taken   BIGINT      DEFAULT 0,
    created_at                    TIMESTAMP   DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_killed_raid_npc_npc_id
    ON player_event_killed_raid_npc (npc_id);
CREATE INDEX IF NOT EXISTS idx_player_event_killed_raid_npc_created_at
    ON player_event_killed_raid_npc (created_at);

-- ============================================================
-- 30. player_event_loot_items
-- EQEmu: player_event_loot_items (13 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_loot_items (
    id           BIGSERIAL PRIMARY KEY,
    item_id      INTEGER     DEFAULT NULL,
    item_name    VARCHAR(64) DEFAULT NULL,
    charges      INTEGER     DEFAULT NULL,
    augment_1_id INTEGER     DEFAULT 0,
    augment_2_id INTEGER     DEFAULT 0,
    augment_3_id INTEGER     DEFAULT 0,
    augment_4_id INTEGER     DEFAULT 0,
    augment_5_id INTEGER     DEFAULT 0,
    augment_6_id INTEGER     DEFAULT 0,
    npc_id       INTEGER     DEFAULT NULL,
    corpse_name  VARCHAR(64) DEFAULT NULL,
    created_at   TIMESTAMP   DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_loot_items_item_npc
    ON player_event_loot_items (item_id, npc_id);
CREATE INDEX IF NOT EXISTS idx_player_event_loot_items_created_at
    ON player_event_loot_items (created_at);

-- ============================================================
-- 31. player_event_merchant_purchase
-- EQEmu: player_event_merchant_purchase (12 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_merchant_purchase (
    id                      BIGSERIAL PRIMARY KEY,
    npc_id                  INTEGER     DEFAULT 0,
    merchant_name           VARCHAR(64) DEFAULT NULL,
    merchant_type           INTEGER     DEFAULT 0,
    item_id                 INTEGER     DEFAULT 0,
    item_name               VARCHAR(64) DEFAULT NULL,
    charges                 INTEGER     DEFAULT 0,
    cost                    INTEGER     DEFAULT 0,
    alternate_currency_id   INTEGER     DEFAULT 0,
    player_money_balance    BIGINT      DEFAULT 0,
    player_currency_balance BIGINT      DEFAULT 0,
    created_at              TIMESTAMP   DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_merchant_purchase_item_npc
    ON player_event_merchant_purchase (item_id, npc_id);
CREATE INDEX IF NOT EXISTS idx_player_event_merchant_purchase_created_at
    ON player_event_merchant_purchase (created_at);

-- ============================================================
-- 32. player_event_merchant_sell
-- EQEmu: player_event_merchant_sell (12 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_merchant_sell (
    id                      BIGSERIAL PRIMARY KEY,
    npc_id                  INTEGER     DEFAULT 0,
    merchant_name           VARCHAR(64) DEFAULT NULL,
    merchant_type           INTEGER     DEFAULT 0,
    item_id                 INTEGER     DEFAULT 0,
    item_name               VARCHAR(64) DEFAULT NULL,
    charges                 INTEGER     DEFAULT 0,
    cost                    INTEGER     DEFAULT 0,
    alternate_currency_id   INTEGER     DEFAULT 0,
    player_money_balance    BIGINT      DEFAULT 0,
    player_currency_balance BIGINT      DEFAULT 0,
    created_at              TIMESTAMP   DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_merchant_sell_item_npc
    ON player_event_merchant_sell (item_id, npc_id);
CREATE INDEX IF NOT EXISTS idx_player_event_merchant_sell_created_at
    ON player_event_merchant_sell (created_at);

-- ============================================================
-- 33. player_event_npc_handin
-- EQEmu: player_event_npc_handin (13 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_npc_handin (
    id              BIGSERIAL PRIMARY KEY,
    npc_id          INTEGER     DEFAULT 0,
    npc_name        VARCHAR(64) DEFAULT NULL,
    handin_copper   BIGINT      DEFAULT 0,
    handin_silver   BIGINT      DEFAULT 0,
    handin_gold     BIGINT      DEFAULT 0,
    handin_platinum BIGINT      DEFAULT 0,
    return_copper   BIGINT      DEFAULT 0,
    return_silver   BIGINT      DEFAULT 0,
    return_gold     BIGINT      DEFAULT 0,
    return_platinum BIGINT      DEFAULT 0,
    is_quest_handin SMALLINT    DEFAULT 0,
    created_at      TIMESTAMP   DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_npc_handin_npc_quest
    ON player_event_npc_handin (npc_id, is_quest_handin);
CREATE INDEX IF NOT EXISTS idx_player_event_npc_handin_created_at
    ON player_event_npc_handin (created_at);

-- ============================================================
-- 34. player_event_npc_handin_entries
-- EQEmu: player_event_npc_handin_entries (14 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_npc_handin_entries (
    id                         BIGSERIAL PRIMARY KEY,
    player_event_npc_handin_id BIGINT    NOT NULL DEFAULT 0,
    "type"                     INTEGER   DEFAULT NULL,
    item_id                    INTEGER   NOT NULL DEFAULT 0,
    charges                    INTEGER   NOT NULL DEFAULT 0,
    evolve_level               INTEGER   NOT NULL DEFAULT 0,
    evolve_amount              BIGINT    NOT NULL DEFAULT 0,
    augment_1_id               INTEGER   NOT NULL DEFAULT 0,
    augment_2_id               INTEGER   NOT NULL DEFAULT 0,
    augment_3_id               INTEGER   NOT NULL DEFAULT 0,
    augment_4_id               INTEGER   NOT NULL DEFAULT 0,
    augment_5_id               INTEGER   NOT NULL DEFAULT 0,
    augment_6_id               INTEGER   NOT NULL DEFAULT 0,
    created_at                 TIMESTAMP DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_npc_handin_entries_type_item
    ON player_event_npc_handin_entries ("type", item_id);
CREATE INDEX IF NOT EXISTS idx_player_event_npc_handin_entries_handin_id
    ON player_event_npc_handin_entries (player_event_npc_handin_id);
CREATE INDEX IF NOT EXISTS idx_player_event_npc_handin_entries_created_at
    ON player_event_npc_handin_entries (created_at);

-- ============================================================
-- 35. player_event_speech
-- EQEmu: player_event_speech (8 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_speech (
    id           BIGSERIAL PRIMARY KEY,
    to_char_id   VARCHAR(64) DEFAULT NULL,
    from_char_id VARCHAR(64) DEFAULT NULL,
    guild_id     INTEGER     DEFAULT 0,
    "type"       INTEGER     DEFAULT 0,
    min_status   INTEGER     DEFAULT 0,
    message      TEXT        DEFAULT NULL,
    created_at   TIMESTAMP   DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_speech_chars
    ON player_event_speech (to_char_id, from_char_id);
CREATE INDEX IF NOT EXISTS idx_player_event_speech_created_at
    ON player_event_speech (created_at);

-- ============================================================
-- 36. player_event_trade
-- EQEmu: player_event_trade (12 columns). Created for 100% MariaDB parity.
-- Note: MariaDB uses int(10) unsigned AUTO_INCREMENT (not bigint).
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_trade (
    id             SERIAL PRIMARY KEY,
    char1_id       INTEGER   DEFAULT 0,
    char2_id       INTEGER   DEFAULT 0,
    char1_copper   BIGINT    DEFAULT 0,
    char1_silver   BIGINT    DEFAULT 0,
    char1_gold     BIGINT    DEFAULT 0,
    char1_platinum BIGINT    DEFAULT 0,
    char2_copper   BIGINT    DEFAULT 0,
    char2_silver   BIGINT    DEFAULT 0,
    char2_gold     BIGINT    DEFAULT 0,
    char2_platinum BIGINT    DEFAULT 0,
    created_at     TIMESTAMP DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_trade_chars
    ON player_event_trade (char1_id, char2_id);
CREATE INDEX IF NOT EXISTS idx_player_event_trade_created_at
    ON player_event_trade (created_at);

-- ============================================================
-- 37. player_event_trade_entries
-- EQEmu: player_event_trade_entries (14 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS player_event_trade_entries (
    id                    BIGSERIAL PRIMARY KEY,
    player_event_trade_id BIGINT    DEFAULT 0,
    char_id               INTEGER   DEFAULT 0,
    "slot"                SMALLINT  DEFAULT 0,
    item_id               INTEGER   DEFAULT 0,
    charges               SMALLINT  DEFAULT 0,
    augment_1_id          INTEGER   DEFAULT 0,
    augment_2_id          INTEGER   DEFAULT 0,
    augment_3_id          INTEGER   DEFAULT 0,
    augment_4_id          INTEGER   DEFAULT 0,
    augment_5_id          INTEGER   DEFAULT 0,
    augment_6_id          INTEGER   DEFAULT 0,
    in_bag                SMALLINT  DEFAULT 0,
    created_at            TIMESTAMP DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_player_event_trade_entries_trade_id
    ON player_event_trade_entries (player_event_trade_id);
CREATE INDEX IF NOT EXISTS idx_player_event_trade_entries_created_at
    ON player_event_trade_entries (created_at);

-- ============================================================
-- 38. spell_buckets
-- EQEmu: spell_buckets (4 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spell_buckets (
    spell_id          INTEGER      NOT NULL,
    bucket_name       VARCHAR(100) NOT NULL DEFAULT '',
    bucket_value      VARCHAR(100) NOT NULL DEFAULT '',
    bucket_comparison SMALLINT     NOT NULL DEFAULT 0,
    PRIMARY KEY (spell_id)
);
CREATE INDEX IF NOT EXISTS idx_spell_buckets_bucket_name ON spell_buckets (bucket_name);

-- ============================================================
-- 39. spire_analytic_event_counts
-- EQEmu/Spire: spire_analytic_event_counts (5 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_analytic_event_counts (
    id         BIGSERIAL PRIMARY KEY,
    event_name VARCHAR(50)  DEFAULT NULL,
    event_key  VARCHAR(120) DEFAULT NULL,
    "count"    BIGINT       DEFAULT NULL,
    updated_at TIMESTAMP    DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_spire_analytic_event_counts_event_name
    ON spire_analytic_event_counts (event_name);
CREATE INDEX IF NOT EXISTS idx_spire_analytic_event_counts_event_name_key
    ON spire_analytic_event_counts (event_name, event_key);

-- ============================================================
-- 40. spire_analytic_events
-- EQEmu/Spire: spire_analytic_events (7 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_analytic_events (
    id          BIGSERIAL PRIMARY KEY,
    event_name  VARCHAR(50)  DEFAULT NULL,
    event_value VARCHAR(200) DEFAULT NULL,
    request_uri VARCHAR(250) DEFAULT NULL,
    ip_address  VARCHAR(20)  DEFAULT NULL,
    user_id     BIGINT       DEFAULT 0,
    updated_at  TIMESTAMP    DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_spire_analytic_events_event_name_ip
    ON spire_analytic_events (event_name, ip_address);
CREATE INDEX IF NOT EXISTS idx_spire_analytic_events_event_name
    ON spire_analytic_events (event_name);

-- ============================================================
-- 41. spire_crash_reports
-- EQEmu/Spire: spire_crash_reports (22 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_crash_reports (
    id                BIGSERIAL PRIMARY KEY,
    platform_name     VARCHAR(20)  DEFAULT NULL,
    origination_info  VARCHAR(150) DEFAULT NULL,
    compile_date      VARCHAR(20)  DEFAULT NULL,
    compile_time      VARCHAR(20)  DEFAULT NULL,
    cpus              BIGINT       DEFAULT NULL,
    crash_report      TEXT         DEFAULT NULL,
    os_machine        VARCHAR(200) DEFAULT NULL,
    os_release        VARCHAR(200) DEFAULT NULL,
    os_sysname        VARCHAR(200) DEFAULT NULL,
    os_version        VARCHAR(200) DEFAULT NULL,
    process_id        BIGINT       DEFAULT NULL,
    rss_memory        DOUBLE PRECISION DEFAULT NULL,
    server_name       VARCHAR(200) DEFAULT NULL,
    server_short_name VARCHAR(200) DEFAULT NULL,
    server_version    VARCHAR(50)  DEFAULT NULL,
    fingerprint       VARCHAR(100) DEFAULT NULL,
    resolved          SMALLINT     DEFAULT 0,
    resolved_by       BIGINT       DEFAULT 0,
    resolved_at       TIMESTAMP    DEFAULT NULL,
    uptime            BIGINT       DEFAULT NULL,
    created_at        TIMESTAMP    DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_spire_crash_reports_version
    ON spire_crash_reports (server_version);
CREATE INDEX IF NOT EXISTS idx_spire_crash_reports_fingerprint
    ON spire_crash_reports (fingerprint);

-- ============================================================
-- 42. spire_server_database_connections
-- EQEmu/Spire: spire_server_database_connections (23 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_server_database_connections (
    id                   BIGSERIAL PRIMARY KEY,
    "name"               VARCHAR(255) DEFAULT NULL,
    db_host              VARCHAR(50)  DEFAULT NULL,
    db_port              VARCHAR(50)  DEFAULT NULL,
    db_name              VARCHAR(50)  DEFAULT NULL,
    db_username          VARCHAR(50)  DEFAULT NULL,
    db_password          VARCHAR(250) DEFAULT NULL,
    content_db_host      VARCHAR(50)  DEFAULT NULL,
    content_db_port      VARCHAR(50)  DEFAULT NULL,
    content_db_name      VARCHAR(50)  DEFAULT NULL,
    content_db_username  VARCHAR(50)  DEFAULT NULL,
    content_db_password  VARCHAR(250) DEFAULT NULL,
    logs_db_host         VARCHAR(50)  DEFAULT NULL,
    logs_db_port         VARCHAR(50)  DEFAULT NULL,
    logs_db_name         VARCHAR(50)  DEFAULT NULL,
    logs_db_username     VARCHAR(50)  DEFAULT NULL,
    logs_db_password     VARCHAR(250) DEFAULT NULL,
    discord_webhook_url  VARCHAR(250) DEFAULT NULL,
    created_from_ip      VARCHAR(50)  DEFAULT NULL,
    created_by           BIGINT       DEFAULT 0,
    created_at           TIMESTAMP    DEFAULT NULL,
    updated_at           TIMESTAMP    DEFAULT NULL,
    deleted_at           TIMESTAMP    DEFAULT NULL
);

-- ============================================================
-- 43. spire_settings
-- EQEmu/Spire: spire_settings (4 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_settings (
    id         BIGSERIAL PRIMARY KEY,
    setting    VARCHAR(190) DEFAULT NULL,
    "value"    VARCHAR(255) DEFAULT NULL,
    created_at TIMESTAMP    DEFAULT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_spire_settings_setting ON spire_settings (setting);

-- ============================================================
-- 44. spire_user_event_log
-- EQEmu/Spire: spire_user_event_log (6 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_user_event_log (
    id                            BIGSERIAL PRIMARY KEY,
    user_id                       BIGINT       DEFAULT NULL,
    server_database_connection_id BIGINT       DEFAULT NULL,
    event_name                    VARCHAR(191) DEFAULT NULL,
    data                          TEXT         DEFAULT NULL,
    created_at                    TIMESTAMP    DEFAULT NULL
);
CREATE INDEX IF NOT EXISTS idx_spire_user_event_log_sdc_id
    ON spire_user_event_log (server_database_connection_id);
CREATE INDEX IF NOT EXISTS idx_spire_user_event_log_sdc_event
    ON spire_user_event_log (server_database_connection_id, event_name);

-- ============================================================
-- 45. spire_user_server_database_connections
-- EQEmu/Spire: spire_user_server_database_connections (8 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_user_server_database_connections (
    id                            BIGSERIAL PRIMARY KEY,
    user_id                       BIGINT    DEFAULT NULL,
    server_database_connection_id BIGINT    DEFAULT NULL,
    active                        BIGINT    DEFAULT 0,
    created_by                    BIGINT    DEFAULT 0,
    created_at                    TIMESTAMP DEFAULT NULL,
    updated_at                    TIMESTAMP DEFAULT NULL,
    deleted_at                    TIMESTAMP DEFAULT NULL
);

-- ============================================================
-- 46. spire_user_server_resource_permissions
-- EQEmu/Spire: spire_user_server_resource_permissions (7 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_user_server_resource_permissions (
    id                            BIGSERIAL PRIMARY KEY,
    user_id                       BIGINT    DEFAULT NULL,
    server_database_connection_id BIGINT    DEFAULT NULL,
    resource_name                 TEXT      DEFAULT NULL,
    can_write                     SMALLINT  DEFAULT NULL,
    can_read                      SMALLINT  DEFAULT NULL,
    created_at                    TIMESTAMP DEFAULT NULL
);

-- ============================================================
-- 47. spire_users
-- EQEmu/Spire: spire_users (14 columns). Created for 100% MariaDB parity.
-- ============================================================
CREATE TABLE IF NOT EXISTS spire_users (
    id                  BIGSERIAL PRIMARY KEY,
    user_name           TEXT      DEFAULT NULL,
    full_name           TEXT      DEFAULT NULL,
    first_name          TEXT      DEFAULT NULL,
    last_name           TEXT      DEFAULT NULL,
    email               TEXT      DEFAULT NULL,
    avatar              TEXT      DEFAULT NULL,
    provider            TEXT      DEFAULT NULL,
    password            TEXT      DEFAULT NULL,
    is_admin            SMALLINT  DEFAULT 0,
    is_server_developer SMALLINT  DEFAULT 0,
    created_at          TIMESTAMP DEFAULT NULL,
    updated_at          TIMESTAMP DEFAULT NULL,
    deleted             TIMESTAMP DEFAULT NULL
);

-- ============================================================
-- 48. trader_audit
-- EQEmu: trader_audit (7 columns). Created for 100% MariaDB parity.
-- No primary key in MariaDB.
-- ============================================================
CREATE TABLE IF NOT EXISTS trader_audit (
    "time"    TIMESTAMP   NOT NULL DEFAULT '1970-01-01 00:00:00',
    seller    VARCHAR(64) NOT NULL DEFAULT '',
    buyer     VARCHAR(64) NOT NULL DEFAULT '',
    itemname  VARCHAR(64) NOT NULL DEFAULT '',
    quantity  INTEGER     NOT NULL DEFAULT 0,
    totalcost INTEGER     NOT NULL DEFAULT 0,
    trantype  SMALLINT    NOT NULL DEFAULT 0
);

COMMIT;
