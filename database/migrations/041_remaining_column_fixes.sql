-- 041_remaining_column_fixes.sql
-- Fix remaining column mismatches found during zone entry testing.
-- Verified against akk-stack MariaDB schemas.

BEGIN;

-- 1. character_corpses: rename gmexp → gm_exp to match C++ repository
ALTER TABLE character_corpses RENAME COLUMN gmexp TO gm_exp;

-- 2. petitions: add ischeckedout column
ALTER TABLE petitions ADD COLUMN IF NOT EXISTS ischeckedout SMALLINT NOT NULL DEFAULT 0;

-- 3. character_buffs: rename ExtraDIChance to lowercase
-- PG created it as "ExtraDIChance" (quoted) but C++ sends unquoted which
-- PG lowercases. Drop the quoted column and add lowercase.
ALTER TABLE character_buffs DROP COLUMN IF EXISTS "ExtraDIChance";
ALTER TABLE character_buffs ADD COLUMN IF NOT EXISTS extradichance INTEGER NOT NULL DEFAULT 0;

-- 4. zone_flags: fix charID casing
-- Migration 040 renamed char_id to "charID" (quoted, case-sensitive).
-- But C++ sends unquoted charID which PG lowercases to charid.
-- Fix: rename to unquoted lowercase that matches PG's folding.
ALTER TABLE zone_flags RENAME COLUMN "charID" TO charid;
ALTER TABLE zone_flags RENAME COLUMN "zoneID" TO zoneid;

-- 5. character_stats_record: add missing skill columns
ALTER TABLE character_stats_record ADD COLUMN IF NOT EXISTS alcohol INTEGER DEFAULT 0;
ALTER TABLE character_stats_record ADD COLUMN IF NOT EXISTS fishing INTEGER DEFAULT 0;
ALTER TABLE character_stats_record ADD COLUMN IF NOT EXISTS tinkering INTEGER DEFAULT 0;

-- 6. completed_shared_tasks: create missing table
CREATE TABLE IF NOT EXISTS completed_shared_tasks (
    id          BIGSERIAL PRIMARY KEY,
    task_id     INTEGER NOT NULL DEFAULT 0,
    accepted_time INTEGER NOT NULL DEFAULT 0,
    expire_time INTEGER NOT NULL DEFAULT 0,
    completion_time INTEGER NOT NULL DEFAULT 0,
    is_locked   SMALLINT NOT NULL DEFAULT 0
);

-- 7. raid_members: add bot_id column
ALTER TABLE raid_members ADD COLUMN IF NOT EXISTS bot_id INTEGER NOT NULL DEFAULT 0;

-- 8. base_data: rename "end" columns to avoid PG reserved word
-- C++ raw SQL uses unquoted "end" which PG rejects.
-- Renaming to "endurance" requires a C++ fix too, so instead we
-- keep the column but the C++ query needs quoting. For now, note
-- this requires a C++ fix in zone_base_data.cpp.
-- (No SQL change here - documented for tracking)

COMMIT;
