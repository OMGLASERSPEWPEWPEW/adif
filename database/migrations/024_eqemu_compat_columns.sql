-- 024_eqemu_compat_columns.sql
-- Add missing EQEmu-compatible columns to tables that were simplified in ADIF.
-- The C++ consumer code expects the full EQEmu column set.

-- character_bind: add is_home column
-- EQEmu uses is_home (0=secondary bind, 1=home) instead of slot
ALTER TABLE character_bind ADD COLUMN IF NOT EXISTS is_home SMALLINT NOT NULL DEFAULT 0;

-- character_buffs: add missing columns the C++ code expects
-- EQEmu: character_buffs has many more columns than ADIF's simplified version
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS slot_id SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS caster_name TEXT NOT NULL DEFAULT '';
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS ticsremaining INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS melee_rune INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS magic_rune INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS persistent SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS "ExtraDIChance" INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS bard_modifier INTEGER NOT NULL DEFAULT 10;
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS bufftype INTEGER NOT NULL DEFAULT 0;

-- character_spells: add id and slot_id columns
-- EQEmu uses id (character_id) and slot_id as composite key
ALTER TABLE character_spells ADD COLUMN IF NOT EXISTS slot_id SMALLINT NOT NULL DEFAULT 0;

-- zone_points: add missing columns
-- EQEmu zone_points has client_version_mask and is_virtual
ALTER TABLE zone_points ADD COLUMN IF NOT EXISTS client_version_mask INTEGER NOT NULL DEFAULT 2147483647;
ALTER TABLE zone_points ADD COLUMN IF NOT EXISTS is_virtual SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE zone_points ADD COLUMN IF NOT EXISTS min_expansion SMALLINT NOT NULL DEFAULT -1;
ALTER TABLE zone_points ADD COLUMN IF NOT EXISTS max_expansion SMALLINT NOT NULL DEFAULT -1;
ALTER TABLE zone_points ADD COLUMN IF NOT EXISTS content_flags TEXT NOT NULL DEFAULT '';
ALTER TABLE zone_points ADD COLUMN IF NOT EXISTS content_flags_disabled TEXT NOT NULL DEFAULT '';
