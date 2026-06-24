-- 031_phase2_schema_fixes.sql
-- Phase 2: Fix column mismatches on inventory, character_currency, doors, guilds,
-- guild_members, faction_values, zone_flags, timers, and character_data edge cases.
--
-- character_data already has all 106 columns (created in a previous session).
-- This migration fixes remaining column gaps and naming mismatches on other tables.
--
-- Prerequisites: 030_phase1_table_renames.sql (table renames already applied).

BEGIN;

-- ============================================================================
-- character_data: Fix column name and type issues
-- ============================================================================
-- RestTimer was created with mixed case (quoted) but C++ generates it unquoted,
-- which PostgreSQL folds to lowercase. Rename to match.
ALTER TABLE character_data RENAME COLUMN "RestTimer" TO resttimer;

-- deleted_at stays as TIMESTAMP — the C++ code uses EXTRACT(EPOCH FROM deleted_at)
-- in SELECT and TO_TIMESTAMP() in INSERT. Changing to INTEGER breaks both.

-- ============================================================================
-- inventory: Add missing columns
-- ============================================================================
-- EQEmu: 17 columns. Currently has 5 (character_id, slot_id, item_id, charges,
-- quantity). Add augment slots, ornaments, guid, etc.

ALTER TABLE inventory ADD COLUMN IF NOT EXISTS color INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS augment_one INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS augment_two INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS augment_three INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS augment_four INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS augment_five INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS augment_six INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS instnodrop SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS custom_data TEXT NOT NULL DEFAULT '';
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS ornament_icon INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS ornament_idfile INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS ornament_hero_model INTEGER NOT NULL DEFAULT 0;
ALTER TABLE inventory ADD COLUMN IF NOT EXISTS guid BIGINT NOT NULL DEFAULT 0;

-- ============================================================================
-- character_currency: Add missing columns
-- ============================================================================
-- EQEmu: 17 columns. Currently has 3 (id, radiant_crystals, ebon_crystals).
-- Add platinum/gold/silver/copper for inventory, bank, and cursor.

ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS platinum INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS gold INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS silver INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS copper INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS platinum_bank INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS gold_bank INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS silver_bank INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS copper_bank INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS platinum_cursor INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS gold_cursor INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS silver_cursor INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS copper_cursor INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS career_radiant_crystals INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_currency ADD COLUMN IF NOT EXISTS career_ebon_crystals INTEGER NOT NULL DEFAULT 0;

-- ============================================================================
-- doors: Add missing columns
-- ============================================================================
-- EQEmu: 37 columns. Currently has 33. Missing: version, guild, disable_timer,
-- dest_instance, buffer, is_ldon_door, dz_switch_id.

ALTER TABLE doors ADD COLUMN IF NOT EXISTS version SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE doors ADD COLUMN IF NOT EXISTS guild SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE doors ADD COLUMN IF NOT EXISTS disable_timer SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE doors ADD COLUMN IF NOT EXISTS dest_instance INTEGER NOT NULL DEFAULT 0;
ALTER TABLE doors ADD COLUMN IF NOT EXISTS buffer REAL NOT NULL DEFAULT 0;
ALTER TABLE doors ADD COLUMN IF NOT EXISTS is_ldon_door SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE doors ADD COLUMN IF NOT EXISTS dz_switch_id INTEGER NOT NULL DEFAULT 0;

-- ============================================================================
-- guilds: Add missing columns + rename
-- ============================================================================
-- EQEmu: 10 columns (id, name, leader, minstatus, motd, tribute, motd_setter,
-- channel, url, favor). Currently has 5 (id, name, leader_id, motd, created_at).

ALTER TABLE guilds RENAME COLUMN leader_id TO leader;
ALTER TABLE guilds ADD COLUMN IF NOT EXISTS minstatus SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE guilds ADD COLUMN IF NOT EXISTS tribute INTEGER NOT NULL DEFAULT 0;
ALTER TABLE guilds ADD COLUMN IF NOT EXISTS motd_setter TEXT NOT NULL DEFAULT '';
ALTER TABLE guilds ADD COLUMN IF NOT EXISTS channel TEXT NOT NULL DEFAULT '';
ALTER TABLE guilds ADD COLUMN IF NOT EXISTS url TEXT NOT NULL DEFAULT '';
ALTER TABLE guilds ADD COLUMN IF NOT EXISTS favor INTEGER NOT NULL DEFAULT 0;

-- ============================================================================
-- guild_members: Add missing column
-- ============================================================================
-- EQEmu: 10 columns. Currently has 9. Missing: online.

ALTER TABLE guild_members ADD COLUMN IF NOT EXISTS online SMALLINT NOT NULL DEFAULT 0;

-- ============================================================================
-- faction_values: Rename id → char_id
-- ============================================================================
-- EQEmu C++ uses char_id, not id.

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'faction_values' AND column_name = 'id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'faction_values' AND column_name = 'char_id'
    ) THEN
        ALTER TABLE faction_values RENAME COLUMN id TO char_id;
    END IF;
END $$;

-- ============================================================================
-- zone_flags: Rename id → char_id
-- ============================================================================

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'zone_flags' AND column_name = 'id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'zone_flags' AND column_name = 'char_id'
    ) THEN
        ALTER TABLE zone_flags RENAME COLUMN id TO char_id;
    END IF;
END $$;

-- ============================================================================
-- timers: Rename id → char_id, fix enable type
-- ============================================================================
-- EQEmu uses char_id and enable as SMALLINT (not BOOLEAN).

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'timers' AND column_name = 'id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'timers' AND column_name = 'char_id'
    ) THEN
        ALTER TABLE timers RENAME COLUMN id TO char_id;
    END IF;
END $$;

ALTER TABLE timers ALTER COLUMN enable DROP DEFAULT;
ALTER TABLE timers
    ALTER COLUMN enable TYPE SMALLINT
    USING (CASE WHEN enable THEN 1 ELSE 0 END);
ALTER TABLE timers ALTER COLUMN enable SET DEFAULT 1;

COMMIT;
