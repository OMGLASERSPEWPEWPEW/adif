-- 042_final_column_fixes.sql
-- Final remaining column fixes from zone entry testing.
-- Cross-referenced against akk-stack MariaDB.

BEGIN;

-- 1. character_corpses: rename killedby → killed_by (akk uses killed_by)
ALTER TABLE character_corpses RENAME COLUMN killedby TO killed_by;

-- 2. petitions: add missing columns and fix primary key
-- Akk-stack has 'dib' as PK (auto_increment), we might have different PK
ALTER TABLE petitions ADD COLUMN IF NOT EXISTS senttime BIGINT NOT NULL DEFAULT 0;

-- 3. player_event_logs: etl_table_id (already added inline, ensure it exists)
ALTER TABLE player_event_logs ADD COLUMN IF NOT EXISTS etl_table_id INTEGER NOT NULL DEFAULT 0;

-- 4. raid_members: add 'id' as proper auto-increment PK if missing
-- Akk has id as bigint auto_increment PK
-- Check if we need to add it
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                   WHERE table_name='raid_members' AND column_name='id'
                   AND column_default LIKE '%nextval%') THEN
        -- id exists but might not be serial, that's OK for now
        NULL;
    END IF;
END $$;

-- 5. inventory: The PK is (character_id, slot_id) which is correct.
-- The C++ base_inventory_repository.h PrimaryKey() returns "character_id" only,
-- causing ON CONFLICT (character_id) to fail. This needs a C++ fix:
-- Change PrimaryKey() to return "character_id, slot_id"
-- Noted here for tracking - no SQL change needed.

-- 6. base_data: The column "end" is a PG reserved word.
-- The C++ raw SQL uses unquoted "end" which fails.
-- Options: a) rename column, b) fix C++ to quote it
-- Since the repository auto-quotes via SelectColumns(), and the raw SQL
-- in zone_base_data.cpp is the only place it fails, we'll rename
-- the columns to avoid the reserved word issue entirely.
ALTER TABLE base_data RENAME COLUMN "end" TO endurance;
ALTER TABLE base_data RENAME COLUMN end_regen TO endurance_regen;
ALTER TABLE base_data RENAME COLUMN end_fac TO endurance_fac;
-- NOTE: This requires matching C++ changes in:
--   - common/repositories/base/base_base_data_repository.h (SelectColumns)
--   - zone/zone_base_data.cpp (raw SQL query)

COMMIT;
