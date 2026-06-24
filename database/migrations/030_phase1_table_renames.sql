-- 030_phase1_table_renames.sql
-- Phase 1: Rename 9 mismatched tables to EQEmu-expected names.
--
-- EQEmu C++ code expects exact table names. ADIF migrations used different names
-- (e.g., "zones" instead of "zone", "npc_templates" instead of "npc_types").
-- Migration 021 created views mapping EQEmu→ADIF names, but views don't support
-- all write operations without INSTEAD OF triggers. Renaming the actual tables
-- is cleaner and more compatible.
--
-- Also drops all FK constraints on character satellite tables so Phase 2 can
-- rebuild character_data without CASCADE-dropping everything.

BEGIN;

-- ============================================================================
-- Step 0: Drop FK constraints from character satellite tables
-- ============================================================================
-- These reference characters(id). Phase 2 will drop+rebuild characters as
-- character_data. Removing FKs lets us drop the table without CASCADE.
-- EQEmu doesn't use FK constraints anyway — the C++ code manages referential
-- integrity in application logic.

ALTER TABLE IF EXISTS character_skills DROP CONSTRAINT IF EXISTS character_skills_character_id_fkey;
ALTER TABLE IF EXISTS character_spells DROP CONSTRAINT IF EXISTS character_spells_character_id_fkey;
ALTER TABLE IF EXISTS character_memmed_spells DROP CONSTRAINT IF EXISTS character_memmed_spells_character_id_fkey;
ALTER TABLE IF EXISTS character_inventory DROP CONSTRAINT IF EXISTS character_inventory_character_id_fkey;
ALTER TABLE IF EXISTS character_buffs DROP CONSTRAINT IF EXISTS character_buffs_character_id_fkey;
ALTER TABLE IF EXISTS character_bind DROP CONSTRAINT IF EXISTS character_bind_id_fkey;
ALTER TABLE IF EXISTS character_currency DROP CONSTRAINT IF EXISTS character_currency_id_fkey;
ALTER TABLE IF EXISTS character_alternate_abilities DROP CONSTRAINT IF EXISTS character_alternate_abilities_id_fkey;
ALTER TABLE IF EXISTS character_faction_values DROP CONSTRAINT IF EXISTS character_faction_values_id_fkey;
ALTER TABLE IF EXISTS character_languages DROP CONSTRAINT IF EXISTS character_languages_id_fkey;
ALTER TABLE IF EXISTS character_material DROP CONSTRAINT IF EXISTS character_material_id_fkey;
ALTER TABLE IF EXISTS character_inspect_messages DROP CONSTRAINT IF EXISTS character_inspect_messages_id_fkey;
ALTER TABLE IF EXISTS character_zone_flags DROP CONSTRAINT IF EXISTS character_zone_flags_id_fkey;
ALTER TABLE IF EXISTS character_timers DROP CONSTRAINT IF EXISTS character_timers_id_fkey;
ALTER TABLE IF EXISTS character_pet_info DROP CONSTRAINT IF EXISTS character_pet_info_char_id_fkey;

-- Also drop FK from characters itself (references accounts and zones)
ALTER TABLE IF EXISTS characters DROP CONSTRAINT IF EXISTS characters_account_id_fkey;
ALTER TABLE IF EXISTS characters DROP CONSTRAINT IF EXISTS characters_zone_id_fkey;

-- ============================================================================
-- Step 1: npc_types (was npc_templates)
-- ============================================================================
-- EQEmu C++: npc_types. ADIF had npc_templates (JSONB). Migration 028 rebuilt
-- npc_templates with all 131 flat columns. Migration 021/028 created a view.
DROP VIEW IF EXISTS npc_types CASCADE;
ALTER TABLE IF EXISTS npc_templates RENAME TO npc_types;

-- ============================================================================
-- Step 2: zone (was zones)
-- ============================================================================
-- EQEmu C++: zone. ADIF had zones. View from 021.
DROP VIEW IF EXISTS zone CASCADE;
ALTER TABLE IF EXISTS zones RENAME TO zone;

-- ============================================================================
-- Step 3: spells_new (was spells)
-- ============================================================================
-- EQEmu C++: spells_new. ADIF had spells. View from 021.
DROP VIEW IF EXISTS spells_new CASCADE;
ALTER TABLE IF EXISTS spells RENAME TO spells_new;

-- ============================================================================
-- Step 4: inventory (was character_inventory)
-- ============================================================================
-- EQEmu C++: inventory. ADIF had character_inventory.
-- FK already dropped above.
ALTER TABLE IF EXISTS character_inventory RENAME TO inventory;

-- ============================================================================
-- Step 5: faction_values (was character_faction_values)
-- ============================================================================
-- EQEmu C++: faction_values. ADIF had character_faction_values.
-- FK already dropped above.
ALTER TABLE IF EXISTS character_faction_values RENAME TO faction_values;

-- ============================================================================
-- Step 6: timers (was character_timers)
-- ============================================================================
-- EQEmu C++: timers. ADIF had character_timers.
-- FK already dropped above.
ALTER TABLE IF EXISTS character_timers RENAME TO timers;

-- ============================================================================
-- Step 7: zone_flags (was character_zone_flags)
-- ============================================================================
-- EQEmu C++: zone_flags. ADIF had character_zone_flags.
-- FK already dropped above.
ALTER TABLE IF EXISTS character_zone_flags RENAME TO zone_flags;

-- ============================================================================
-- Step 8: faction_base_data (was factions)
-- ============================================================================
-- EQEmu C++: faction_base_data (client_faction_id, min, max, unk_hero1-3).
-- ADIF factions had (id, name) — completely different schema. Drop and rebuild.
DROP TABLE IF EXISTS factions CASCADE;
CREATE TABLE IF NOT EXISTS faction_base_data (
    client_faction_id SMALLINT NOT NULL PRIMARY KEY,
    min               SMALLINT NOT NULL DEFAULT 0,
    max               SMALLINT NOT NULL DEFAULT 0,
    unk_hero1         SMALLINT NOT NULL DEFAULT 0,
    unk_hero2         SMALLINT NOT NULL DEFAULT 0,
    unk_hero3         SMALLINT NOT NULL DEFAULT 0
);

-- ============================================================================
-- Step 9: faction_association (was npc_faction_associations)
-- ============================================================================
-- EQEmu C++: faction_association (21 columns: id, id_1..id_10, mod_1..mod_10).
-- ADIF npc_faction_associations had (npc_faction_id, faction_id, value) —
-- completely different schema. Drop and rebuild.
DROP TABLE IF EXISTS npc_faction_associations CASCADE;
CREATE TABLE IF NOT EXISTS faction_association (
    id    INTEGER NOT NULL PRIMARY KEY,
    id_1  INTEGER NOT NULL DEFAULT 0,
    mod_1 REAL NOT NULL DEFAULT 0,
    id_2  INTEGER NOT NULL DEFAULT 0,
    mod_2 REAL NOT NULL DEFAULT 0,
    id_3  INTEGER NOT NULL DEFAULT 0,
    mod_3 REAL NOT NULL DEFAULT 0,
    id_4  INTEGER NOT NULL DEFAULT 0,
    mod_4 REAL NOT NULL DEFAULT 0,
    id_5  INTEGER NOT NULL DEFAULT 0,
    mod_5 REAL NOT NULL DEFAULT 0,
    id_6  INTEGER NOT NULL DEFAULT 0,
    mod_6 REAL NOT NULL DEFAULT 0,
    id_7  INTEGER NOT NULL DEFAULT 0,
    mod_7 REAL NOT NULL DEFAULT 0,
    id_8  INTEGER NOT NULL DEFAULT 0,
    mod_8 REAL NOT NULL DEFAULT 0,
    id_9  INTEGER NOT NULL DEFAULT 0,
    mod_9 REAL NOT NULL DEFAULT 0,
    id_10 INTEGER NOT NULL DEFAULT 0,
    mod_10 REAL NOT NULL DEFAULT 0
);

COMMIT;
