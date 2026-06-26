-- Migration 044: Fix all column mismatches for 100% MariaDB parity
-- Adds missing columns, fixes type mismatches, corrects PKs
-- Source: akk-stack MariaDB (peq) ground truth, 2026-06-26

-- Using individual statements (no transaction) so partial progress is preserved
-- Boolean→SMALLINT conversions require DROP DEFAULT before type change

-- ============================================================
-- 1. buyer_buy_lines: add missing columns, rename columns, widen types
-- EQEmu: buyer_buy_lines has char_id, buy_slot_id, item_qty, item_icon
-- ADIF had: buy_slot (not buy_slot_id), item_quantity (not item_qty),
--           missing char_id and item_icon, id/buyer_id as INTEGER not BIGINT
-- ============================================================
ALTER TABLE buyer_buy_lines ADD COLUMN IF NOT EXISTS char_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE buyer_buy_lines RENAME COLUMN buy_slot TO buy_slot_id;
ALTER TABLE buyer_buy_lines RENAME COLUMN item_quantity TO item_qty;
ALTER TABLE buyer_buy_lines ADD COLUMN IF NOT EXISTS item_icon INTEGER NOT NULL DEFAULT 0;
ALTER TABLE buyer_buy_lines ALTER COLUMN id TYPE BIGINT;
ALTER TABLE buyer_buy_lines ALTER COLUMN buyer_id TYPE BIGINT;

-- ============================================================
-- 2. buyer_trade_items: rename columns, widen types, drop extra column
-- EQEmu: buyer_trade_items has buyer_buy_lines_id (not buyer_id),
--        item_qty (not item_quantity), no buy_slot column
-- ============================================================
ALTER TABLE buyer_trade_items RENAME COLUMN buyer_id TO buyer_buy_lines_id;
ALTER TABLE buyer_trade_items ALTER COLUMN buyer_buy_lines_id TYPE BIGINT;
ALTER TABLE buyer_trade_items DROP COLUMN IF EXISTS buy_slot;
ALTER TABLE buyer_trade_items RENAME COLUMN item_quantity TO item_qty;
ALTER TABLE buyer_trade_items ALTER COLUMN id TYPE BIGINT;

-- ============================================================
-- 3. char_create_combinations: add expansions_req, widen types, fix PK
-- EQEmu: PK is (class, deity, race, start_zone), types are INT not SMALLINT
-- ADIF: had SMALLINT types and id-only PK
-- ============================================================
ALTER TABLE char_create_combinations ADD COLUMN IF NOT EXISTS expansions_req INTEGER NOT NULL DEFAULT 0;
ALTER TABLE char_create_combinations ALTER COLUMN race TYPE INTEGER;
ALTER TABLE char_create_combinations ALTER COLUMN class TYPE INTEGER;
ALTER TABLE char_create_combinations ALTER COLUMN deity TYPE INTEGER;
ALTER TABLE char_create_combinations ALTER COLUMN start_zone TYPE INTEGER;

-- Drop old PK (id) and add composite PK matching MySQL
DO $$
BEGIN
    -- Drop existing PK constraint
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'char_create_combinations_pkey'
               AND conrelid = 'char_create_combinations'::regclass) THEN
        ALTER TABLE char_create_combinations DROP CONSTRAINT char_create_combinations_pkey;
    END IF;
    -- Add composite PK (keep id column but it's no longer PK)
    ALTER TABLE char_create_combinations ADD PRIMARY KEY (class, deity, race, start_zone);
EXCEPTION WHEN duplicate_table THEN NULL;
END $$;

-- ============================================================
-- 4. char_create_point_allocations: widen stat columns from SMALLINT to INTEGER
-- EQEmu: all stat columns are INT(11), not SMALLINT
-- ============================================================
ALTER TABLE char_create_point_allocations ALTER COLUMN base_str TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN base_sta TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN base_dex TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN base_agi TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN base_int TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN base_wis TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN base_cha TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN alloc_str TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN alloc_sta TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN alloc_dex TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN alloc_agi TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN alloc_int TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN alloc_wis TYPE INTEGER;
ALTER TABLE char_create_point_allocations ALTER COLUMN alloc_cha TYPE INTEGER;

-- ============================================================
-- 5. character_corpse_items: add 11 missing columns, widen types
-- EQEmu: has aug_1-6, attuned, custom_data, ornamenticon,
--        ornamentidfile, ornament_hero_model, equip_slot/charges as INT
-- ADIF: missing all augment/ornament columns
-- ============================================================
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS aug_1 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS aug_2 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS aug_3 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS aug_4 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS aug_5 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS aug_6 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS attuned SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS custom_data TEXT DEFAULT NULL;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS ornamenticon INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS ornamentidfile INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ADD COLUMN IF NOT EXISTS ornament_hero_model INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpse_items ALTER COLUMN equip_slot TYPE INTEGER;
ALTER TABLE character_corpse_items ALTER COLUMN charges TYPE INTEGER;

-- ============================================================
-- 6. character_corpses: fix 20 column types for MariaDB parity
-- EQEmu: uses tinyint (SMALLINT) for booleans, int for most fields
-- ADIF: had BOOLEAN, REAL, SMALLINT where MySQL uses INT/TINYINT
-- ============================================================
-- Boolean → SMALLINT (MySQL tinyint) — must drop default first, then re-add
ALTER TABLE character_corpses ALTER COLUMN is_rezzed DROP DEFAULT;
ALTER TABLE character_corpses ALTER COLUMN is_rezzed TYPE SMALLINT USING (CASE WHEN is_rezzed THEN 1 ELSE 0 END);
ALTER TABLE character_corpses ALTER COLUMN is_rezzed SET DEFAULT 0;

ALTER TABLE character_corpses ALTER COLUMN is_buried DROP DEFAULT;
ALTER TABLE character_corpses ALTER COLUMN is_buried TYPE SMALLINT USING (CASE WHEN is_buried THEN 1 ELSE 0 END);
ALTER TABLE character_corpses ALTER COLUMN is_buried SET DEFAULT 0;

ALTER TABLE character_corpses ALTER COLUMN is_locked DROP DEFAULT;
ALTER TABLE character_corpses ALTER COLUMN is_locked TYPE SMALLINT USING (CASE WHEN is_locked THEN 1 ELSE 0 END);
ALTER TABLE character_corpses ALTER COLUMN is_locked SET DEFAULT 0;

ALTER TABLE character_corpses ALTER COLUMN rezzable DROP DEFAULT;
ALTER TABLE character_corpses ALTER COLUMN rezzable TYPE SMALLINT USING (CASE WHEN rezzable THEN 1 ELSE 0 END);
ALTER TABLE character_corpses ALTER COLUMN rezzable SET DEFAULT 0;
-- REAL → INTEGER
ALTER TABLE character_corpses ALTER COLUMN size TYPE INTEGER USING size::INTEGER;
-- SMALLINT → INTEGER
ALTER TABLE character_corpses ALTER COLUMN level TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN race TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN gender TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN class TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN deity TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN texture TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN helm_texture TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN hair_color TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN beard_color TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN eye_color_1 TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN eye_color_2 TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN hair_style TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN face TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN beard TYPE INTEGER;
ALTER TABLE character_corpses ALTER COLUMN killed_by TYPE INTEGER;

-- ============================================================
-- 7. character_pet_buffs: add 3 missing columns, widen types
-- EQEmu: has numhits, rune, instrument_mod; pet/slot are INT
-- ============================================================
ALTER TABLE character_pet_buffs ADD COLUMN IF NOT EXISTS numhits INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_pet_buffs ADD COLUMN IF NOT EXISTS rune INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_pet_buffs ADD COLUMN IF NOT EXISTS instrument_mod SMALLINT NOT NULL DEFAULT 10;
ALTER TABLE character_pet_buffs ALTER COLUMN pet TYPE INTEGER;
ALTER TABLE character_pet_buffs ALTER COLUMN slot TYPE INTEGER;

-- ============================================================
-- 8. character_pet_info: widen pet column
-- EQEmu: pet is INT, ADIF had SMALLINT
-- ============================================================
ALTER TABLE character_pet_info ALTER COLUMN pet TYPE INTEGER;

-- ============================================================
-- 9. character_pet_inventory: widen pet and slot columns
-- EQEmu: pet/slot are INT, ADIF had SMALLINT
-- ============================================================
ALTER TABLE character_pet_inventory ALTER COLUMN pet TYPE INTEGER;
ALTER TABLE character_pet_inventory ALTER COLUMN slot TYPE INTEGER;

-- ============================================================
-- 10. character_stats_record: add updated_at, widen types, fix PK
-- EQEmu: hp/mana/endurance are BIGINT, has updated_at DATETIME,
--        created_at is DATETIME (not INTEGER), PK is character_id
-- ============================================================
ALTER TABLE character_stats_record ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP DEFAULT NULL;
ALTER TABLE character_stats_record ALTER COLUMN hp TYPE BIGINT;
ALTER TABLE character_stats_record ALTER COLUMN mana TYPE BIGINT;
ALTER TABLE character_stats_record ALTER COLUMN endurance TYPE BIGINT;
-- Convert created_at from INTEGER (unix epoch) to TIMESTAMP
ALTER TABLE character_stats_record ALTER COLUMN created_at TYPE TIMESTAMP
    USING TO_TIMESTAMP(created_at) AT TIME ZONE 'UTC';
-- Drop poison_making if it exists (not in MySQL schema)
ALTER TABLE character_stats_record DROP COLUMN IF EXISTS poison_making;
-- Add PK on character_id if not already present
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'character_stats_record_pkey'
                   AND conrelid = 'character_stats_record'::regclass) THEN
        ALTER TABLE character_stats_record ADD PRIMARY KEY (character_id);
    END IF;
EXCEPTION WHEN duplicate_table THEN NULL;
END $$;

-- ============================================================
-- 11. completed_shared_tasks: convert time columns to TIMESTAMP
-- EQEmu: accepted_time, expire_time, completion_time are DATETIME
-- ADIF: had INTEGER (unix epoch)
-- ============================================================
ALTER TABLE completed_shared_tasks ALTER COLUMN accepted_time TYPE TIMESTAMP
    USING TO_TIMESTAMP(accepted_time) AT TIME ZONE 'UTC';
ALTER TABLE completed_shared_tasks ALTER COLUMN expire_time TYPE TIMESTAMP
    USING TO_TIMESTAMP(expire_time) AT TIME ZONE 'UTC';
ALTER TABLE completed_shared_tasks ALTER COLUMN completion_time TYPE TIMESTAMP
    USING TO_TIMESTAMP(completion_time) AT TIME ZONE 'UTC';

-- ============================================================
-- 12. data_buckets: widen id, account_id, character_id to BIGINT
-- EQEmu: all three are BIGINT(20), ADIF had INTEGER
-- ============================================================
ALTER TABLE data_buckets ALTER COLUMN id TYPE BIGINT;
ALTER TABLE data_buckets ALTER COLUMN account_id TYPE BIGINT;
ALTER TABLE data_buckets ALTER COLUMN character_id TYPE BIGINT;

-- ============================================================
-- 13. group_id: rename groupid → group_id, drop accountid, fix PK
-- EQEmu: column is group_id (not groupid), no accountid column,
--        PK is (bot_id, character_id, group_id, merc_id)
-- ============================================================
ALTER TABLE group_id RENAME COLUMN groupid TO group_id;
ALTER TABLE group_id DROP COLUMN IF EXISTS accountid;

DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'group_id_pkey'
               AND conrelid = 'group_id'::regclass) THEN
        ALTER TABLE group_id DROP CONSTRAINT group_id_pkey;
    END IF;
    ALTER TABLE group_id ADD PRIMARY KEY (bot_id, character_id, group_id, merc_id);
EXCEPTION WHEN duplicate_table THEN NULL;
END $$;

-- ============================================================
-- 14. group_leaders: add 7 missing columns
-- EQEmu: has marknpc, leadershipaa, maintank, assist, puller,
--        mentoree, mentor_percent
-- ADIF: missing all of these
-- ============================================================
ALTER TABLE group_leaders ADD COLUMN IF NOT EXISTS marknpc VARCHAR(64) NOT NULL DEFAULT '';
ALTER TABLE group_leaders ADD COLUMN IF NOT EXISTS leadershipaa BYTEA DEFAULT NULL;
ALTER TABLE group_leaders ADD COLUMN IF NOT EXISTS maintank VARCHAR(64) NOT NULL DEFAULT '';
ALTER TABLE group_leaders ADD COLUMN IF NOT EXISTS assist VARCHAR(64) NOT NULL DEFAULT '';
ALTER TABLE group_leaders ADD COLUMN IF NOT EXISTS puller VARCHAR(64) NOT NULL DEFAULT '';
ALTER TABLE group_leaders ADD COLUMN IF NOT EXISTS mentoree VARCHAR(64) NOT NULL DEFAULT '';
ALTER TABLE group_leaders ADD COLUMN IF NOT EXISTS mentor_percent INTEGER NOT NULL DEFAULT 0;

-- ============================================================
-- 15. inventory_snapshots: add missing guid column
-- EQEmu: has guid BIGINT, ADIF was missing it
-- ============================================================
ALTER TABLE inventory_snapshots ADD COLUMN IF NOT EXISTS guid BIGINT NOT NULL DEFAULT 0;

-- ============================================================
-- 16. raid_details: widen types, add 10 missing columns
-- EQEmu: loottype is INT, locked is TINYINT, has motd and
--        marked_npc_1/2/3 entity/zone/instance columns
-- ============================================================
ALTER TABLE raid_details ALTER COLUMN loottype TYPE INTEGER;
ALTER TABLE raid_details ALTER COLUMN locked DROP DEFAULT;
ALTER TABLE raid_details ALTER COLUMN locked TYPE SMALLINT USING (CASE WHEN locked THEN 1 ELSE 0 END);
ALTER TABLE raid_details ALTER COLUMN locked SET DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS motd VARCHAR(1024) DEFAULT NULL;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_1_entity_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_1_zone_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_1_instance_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_2_entity_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_2_zone_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_2_instance_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_3_entity_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_3_zone_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE raid_details ADD COLUMN IF NOT EXISTS marked_npc_3_instance_id INTEGER NOT NULL DEFAULT 0;

-- ============================================================
-- 17. raid_leaders: add 2 missing columns
-- EQEmu: has mentoree and mentor_percent
-- ============================================================
ALTER TABLE raid_leaders ADD COLUMN IF NOT EXISTS mentoree VARCHAR(64) NOT NULL DEFAULT '';
ALTER TABLE raid_leaders ADD COLUMN IF NOT EXISTS mentor_percent INTEGER NOT NULL DEFAULT 0;

-- ============================================================
-- 18. raid_members: add 3 missing columns, widen id, fix boolean types
-- EQEmu: has is_marker, is_assister, note; id is BIGINT;
--        isgroupleader/israidleader/islooter are TINYINT not BOOLEAN
-- ============================================================
ALTER TABLE raid_members ADD COLUMN IF NOT EXISTS is_marker SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE raid_members ADD COLUMN IF NOT EXISTS is_assister SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE raid_members ADD COLUMN IF NOT EXISTS note VARCHAR(64) NOT NULL DEFAULT '';
ALTER TABLE raid_members ALTER COLUMN id TYPE BIGINT;
ALTER TABLE raid_members ALTER COLUMN isgroupleader DROP DEFAULT;
ALTER TABLE raid_members ALTER COLUMN isgroupleader TYPE SMALLINT USING (CASE WHEN isgroupleader THEN 1 ELSE 0 END);
ALTER TABLE raid_members ALTER COLUMN isgroupleader SET DEFAULT 0;

ALTER TABLE raid_members ALTER COLUMN israidleader DROP DEFAULT;
ALTER TABLE raid_members ALTER COLUMN israidleader TYPE SMALLINT USING (CASE WHEN israidleader THEN 1 ELSE 0 END);
ALTER TABLE raid_members ALTER COLUMN israidleader SET DEFAULT 0;

ALTER TABLE raid_members ALTER COLUMN islooter DROP DEFAULT;
ALTER TABLE raid_members ALTER COLUMN islooter TYPE SMALLINT USING (CASE WHEN islooter THEN 1 ELSE 0 END);
ALTER TABLE raid_members ALTER COLUMN islooter SET DEFAULT 0;

-- ============================================================
-- 19. trader: MAJOR rebuild — drop and recreate to match MySQL schema
-- EQEmu: trader has 19 columns including aug_slot_1-6, item_sn,
--        item_charges, char_entity_id, char_zone_id, etc.
-- ADIF: had only 6 columns with a different PK structure
-- ============================================================
DROP TABLE IF EXISTS trader;
CREATE TABLE trader (
    id                      BIGSERIAL PRIMARY KEY,
    char_id                 INTEGER NOT NULL DEFAULT 0,
    item_id                 INTEGER NOT NULL DEFAULT 0,
    serialnumber            INTEGER NOT NULL DEFAULT 0,
    charges                 INTEGER NOT NULL DEFAULT 0,
    item_cost               INTEGER NOT NULL DEFAULT 0,
    aug_slot_1              INTEGER NOT NULL DEFAULT 0,
    aug_slot_2              INTEGER NOT NULL DEFAULT 0,
    aug_slot_3              INTEGER NOT NULL DEFAULT 0,
    aug_slot_4              INTEGER NOT NULL DEFAULT 0,
    aug_slot_5              INTEGER NOT NULL DEFAULT 0,
    aug_slot_6              INTEGER NOT NULL DEFAULT 0,
    item_sn                 INTEGER NOT NULL DEFAULT 0,
    item_charges            INTEGER NOT NULL DEFAULT 0,
    char_entity_id          INTEGER NOT NULL DEFAULT 0,
    char_zone_id            INTEGER NOT NULL DEFAULT 0,
    char_zone_instance_id   INTEGER NOT NULL DEFAULT 0,
    active_transaction      SMALLINT NOT NULL DEFAULT 0,
    listing_date            TIMESTAMP DEFAULT NULL
);
CREATE INDEX idx_trader_char_id ON trader (char_id);

-- ============================================================
-- 20. adventure_template: fix graveyard_radius type (TEXT → REAL)
-- EQEmu: graveyard_radius is FLOAT, ADIF had TEXT (bug)
-- ============================================================
ALTER TABLE adventure_template ALTER COLUMN graveyard_radius TYPE REAL USING graveyard_radius::REAL;

-- ============================================================
-- 21. items_evolving_details: widen types, fix sub_type mismatch
-- EQEmu: item_evolve_level/type are INT, sub_type is VARCHAR(200),
--        required_amount is BIGINT
-- ADIF: had SMALLINT for all, sub_type was SMALLINT (wrong type entirely)
-- ============================================================
ALTER TABLE items_evolving_details ALTER COLUMN item_evolve_level TYPE INTEGER;
ALTER TABLE items_evolving_details ALTER COLUMN type TYPE INTEGER;
ALTER TABLE items_evolving_details ALTER COLUMN sub_type TYPE VARCHAR(200) USING sub_type::VARCHAR(200);
ALTER TABLE items_evolving_details ALTER COLUMN required_amount TYPE BIGINT;

-- ============================================================
-- 22. Primary key fixes for various tables
-- Correcting composite PKs to match MySQL schema
-- ============================================================

-- adventure_template_entry: PK should be (id, template_id)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'adventure_template_entry_pkey'
               AND conrelid = 'adventure_template_entry'::regclass) THEN
        ALTER TABLE adventure_template_entry DROP CONSTRAINT adventure_template_entry_pkey;
    END IF;
    ALTER TABLE adventure_template_entry ADD PRIMARY KEY (id, template_id);
EXCEPTION WHEN undefined_table THEN NULL;
END $$;

-- ldon_trap_entries: PK should be (id, trap_id)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'ldon_trap_entries_pkey'
               AND conrelid = 'ldon_trap_entries'::regclass) THEN
        ALTER TABLE ldon_trap_entries DROP CONSTRAINT ldon_trap_entries_pkey;
    END IF;
    ALTER TABLE ldon_trap_entries ADD PRIMARY KEY (id, trap_id);
EXCEPTION WHEN undefined_table THEN NULL;
END $$;

-- merchantlist_temp: PK should be (instance_id, npcid, slot, zone_id)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'merchantlist_temp_pkey'
               AND conrelid = 'merchantlist_temp'::regclass) THEN
        ALTER TABLE merchantlist_temp DROP CONSTRAINT merchantlist_temp_pkey;
    END IF;
    ALTER TABLE merchantlist_temp ADD PRIMARY KEY (instance_id, npcid, slot, zone_id);
EXCEPTION WHEN undefined_table THEN NULL;
END $$;

-- object_contents: PK should be (bagidx, parentid) — remove zoneid from PK
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'object_contents_pkey'
               AND conrelid = 'object_contents'::regclass) THEN
        ALTER TABLE object_contents DROP CONSTRAINT object_contents_pkey;
    END IF;
    ALTER TABLE object_contents ADD PRIMARY KEY (bagidx, parentid);
EXCEPTION WHEN undefined_table THEN NULL;
END $$;

-- tradeskill_recipe: add PK (id)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'tradeskill_recipe_pkey'
                   AND conrelid = 'tradeskill_recipe'::regclass) THEN
        ALTER TABLE tradeskill_recipe ADD PRIMARY KEY (id);
    END IF;
EXCEPTION WHEN undefined_table THEN NULL;
END $$;

-- tradeskill_recipe_entries: add PK (id)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'tradeskill_recipe_entries_pkey'
                   AND conrelid = 'tradeskill_recipe_entries'::regclass) THEN
        ALTER TABLE tradeskill_recipe_entries ADD PRIMARY KEY (id);
    END IF;
EXCEPTION WHEN undefined_table THEN NULL;
END $$;

-- zone_points: add PK (id)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'zone_points_pkey'
                   AND conrelid = 'zone_points'::regclass) THEN
        ALTER TABLE zone_points ADD PRIMARY KEY (id);
    END IF;
EXCEPTION WHEN undefined_table THEN NULL;
END $$;

-- tributes: PK should be (id, isguild)
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'tributes_pkey'
               AND conrelid = 'tributes'::regclass) THEN
        ALTER TABLE tributes DROP CONSTRAINT tributes_pkey;
    END IF;
    ALTER TABLE tributes ADD PRIMARY KEY (id, isguild);
EXCEPTION WHEN undefined_table THEN NULL;
END $$;

-- ============================================================
-- 23. Various type fixes across many tables (safe widening for parity)
-- These are all safe type widenings or corrections to match MySQL
-- ============================================================

-- instance_list: expire_at → BIGINT
ALTER TABLE instance_list ALTER COLUMN expire_at TYPE BIGINT;

-- spawn2_disabled: id → BIGINT
ALTER TABLE spawn2_disabled ALTER COLUMN id TYPE BIGINT;

-- zone_state_spawns: id, hp, mana, endurance → BIGINT
ALTER TABLE zone_state_spawns ALTER COLUMN id TYPE BIGINT;
ALTER TABLE zone_state_spawns ALTER COLUMN hp TYPE BIGINT;
ALTER TABLE zone_state_spawns ALTER COLUMN mana TYPE BIGINT;
ALTER TABLE zone_state_spawns ALTER COLUMN endurance TYPE BIGINT;

-- player_event_log_settings: id → BIGINT
ALTER TABLE player_event_log_settings ALTER COLUMN id TYPE BIGINT;

-- player_event_logs: account_id, character_id, etl_table_id → BIGINT
ALTER TABLE player_event_logs ALTER COLUMN account_id TYPE BIGINT;
ALTER TABLE player_event_logs ALTER COLUMN character_id TYPE BIGINT;
ALTER TABLE player_event_logs ALTER COLUMN etl_table_id TYPE BIGINT;

-- character_evolving_items: id → BIGINT
ALTER TABLE character_evolving_items ALTER COLUMN id TYPE BIGINT;

-- npc_scale_global_base: spell_scale, heal_scale → INTEGER (from REAL)
ALTER TABLE npc_scale_global_base ALTER COLUMN spell_scale TYPE INTEGER USING spell_scale::INTEGER;
ALTER TABLE npc_scale_global_base ALTER COLUMN heal_scale TYPE INTEGER USING heal_scale::INTEGER;

-- task_activities: activityid → INTEGER (from SMALLINT)
ALTER TABLE task_activities ALTER COLUMN activityid TYPE INTEGER;

-- character_bind: slot → INTEGER (from SMALLINT)
ALTER TABLE character_bind ALTER COLUMN slot TYPE INTEGER;

-- level_exp_mods: level → INTEGER (from SMALLINT)
ALTER TABLE level_exp_mods ALTER COLUMN level TYPE INTEGER;

-- start_zones: widen SMALLINT columns → INTEGER
ALTER TABLE start_zones ALTER COLUMN player_choice TYPE INTEGER;
ALTER TABLE start_zones ALTER COLUMN player_class TYPE INTEGER;
ALTER TABLE start_zones ALTER COLUMN player_deity TYPE INTEGER;
ALTER TABLE start_zones ALTER COLUMN player_race TYPE INTEGER;
ALTER TABLE start_zones ALTER COLUMN start_zone TYPE INTEGER;

-- inventory: slot_id → INTEGER (from SMALLINT)
ALTER TABLE inventory ALTER COLUMN slot_id TYPE INTEGER;

-- bugs: date → DATE (from TIMESTAMP)
ALTER TABLE bugs ALTER COLUMN date TYPE DATE USING date::DATE;

-- guild_tributes: enabled → INTEGER (from SMALLINT)
ALTER TABLE guild_tributes ALTER COLUMN enabled TYPE INTEGER;

-- spawn_events: enabled, strict → SMALLINT (from BOOLEAN)
ALTER TABLE spawn_events ALTER COLUMN enabled DROP DEFAULT;
ALTER TABLE spawn_events ALTER COLUMN enabled TYPE SMALLINT USING (CASE WHEN enabled THEN 1 ELSE 0 END);
ALTER TABLE spawn_events ALTER COLUMN enabled SET DEFAULT 1;

ALTER TABLE spawn_events ALTER COLUMN strict DROP DEFAULT;
ALTER TABLE spawn_events ALTER COLUMN strict TYPE SMALLINT USING (CASE WHEN strict THEN 1 ELSE 0 END);
ALTER TABLE spawn_events ALTER COLUMN strict SET DEFAULT 0;

-- petitions: widen SMALLINT → INTEGER, senttime → BIGINT
ALTER TABLE petitions ALTER COLUMN urgency TYPE INTEGER;
ALTER TABLE petitions ALTER COLUMN charclass TYPE INTEGER;
ALTER TABLE petitions ALTER COLUMN charrace TYPE INTEGER;
ALTER TABLE petitions ALTER COLUMN charlevel TYPE INTEGER;
ALTER TABLE petitions ALTER COLUMN checkouts TYPE INTEGER;
ALTER TABLE petitions ALTER COLUMN senttime TYPE BIGINT;
