-- Migration 045: Fix runtime errors from zone entry + zoning
-- Source: zone server logs from first successful Grobb → Innothule zone transition
-- 3 issues: integer overflow, id=0 auto-increment, composite PK mismatch

-- 1. character_data: tribute_time_remaining overflows INTEGER with uint32_t max (4294967295)
-- EQEmu C++ uses uint32_t for this field. PostgreSQL INTEGER is signed (max 2147483647).
ALTER TABLE character_data ALTER COLUMN tribute_time_remaining TYPE BIGINT;

-- 2. player_event_logs: C++ inserts id=0, PostgreSQL treats 0 as real value
-- Same pattern as character_data fix — trigger converts 0 to next sequence value
CREATE OR REPLACE FUNCTION auto_id_on_zero() RETURNS trigger AS $$
BEGIN
  IF NEW.id = 0 THEN
    NEW.id := nextval(pg_get_serial_sequence(TG_TABLE_NAME, 'id'));
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_player_event_logs_auto_id
  BEFORE INSERT ON player_event_logs
  FOR EACH ROW EXECUTE FUNCTION auto_id_on_zero();

-- 3. respawn_times: PK is (id, instance_id) but was missing in some migration paths
-- Ensure the composite PK exists
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'respawn_times_pkey'
                   AND conrelid = 'respawn_times'::regclass) THEN
        ALTER TABLE respawn_times ADD PRIMARY KEY (id, instance_id);
    END IF;
EXCEPTION WHEN duplicate_table THEN NULL;
END $$;

-- 4. character_exp_modifiers: ensure composite PK exists
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'character_exp_modifiers_pkey'
               AND conrelid = 'character_exp_modifiers'::regclass) THEN
        ALTER TABLE character_exp_modifiers DROP CONSTRAINT character_exp_modifiers_pkey;
    END IF;
    ALTER TABLE character_exp_modifiers ADD PRIMARY KEY (character_id, zone_id, instance_version);
EXCEPTION WHEN duplicate_table THEN NULL;
END $$;
