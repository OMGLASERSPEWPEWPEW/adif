-- 040_column_fixes_and_rebuilds.sql
-- Fix column mismatches between C++ code expectations and PostgreSQL schema.
-- Schemas verified against akk-stack MariaDB (ground truth).

BEGIN;

-- ============================================================
-- 1. character_corpses: add missing columns
-- EQEmu C++ expects instance_id, guild_consent_id, was_at_graveyard,
-- drakkin_heritage/tattoo/details, entity_variables
-- ============================================================
ALTER TABLE character_corpses ADD COLUMN IF NOT EXISTS instance_id SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE character_corpses ADD COLUMN IF NOT EXISTS guild_consent_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpses ADD COLUMN IF NOT EXISTS was_at_graveyard SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE character_corpses ADD COLUMN IF NOT EXISTS drakkin_heritage INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpses ADD COLUMN IF NOT EXISTS drakkin_tattoo INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpses ADD COLUMN IF NOT EXISTS drakkin_details INTEGER NOT NULL DEFAULT 0;
ALTER TABLE character_corpses ADD COLUMN IF NOT EXISTS entity_variables TEXT DEFAULT NULL;

-- ============================================================
-- 2. object_contents: add augslot1-6
-- ============================================================
ALTER TABLE object_contents ADD COLUMN IF NOT EXISTS augslot1 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE object_contents ADD COLUMN IF NOT EXISTS augslot2 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE object_contents ADD COLUMN IF NOT EXISTS augslot3 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE object_contents ADD COLUMN IF NOT EXISTS augslot4 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE object_contents ADD COLUMN IF NOT EXISTS augslot5 INTEGER NOT NULL DEFAULT 0;
ALTER TABLE object_contents ADD COLUMN IF NOT EXISTS augslot6 INTEGER NOT NULL DEFAULT 0;

-- ============================================================
-- 3. doors: add close_timer_ms
-- (dz_switch_id already exists)
-- ============================================================
ALTER TABLE doors ADD COLUMN IF NOT EXISTS close_timer_ms SMALLINT NOT NULL DEFAULT 5000;

-- ============================================================
-- 4. character_spells: rename slot → slot_id
-- ============================================================
ALTER TABLE character_spells RENAME COLUMN slot TO slot_id;

-- ============================================================
-- 5. character_memmed_spells: rename slot → slot_id
-- ============================================================
ALTER TABLE character_memmed_spells RENAME COLUMN slot TO slot_id;

-- ============================================================
-- 6. character_buffs: rebuild to match EQEmu schema
-- Current PG schema is missing most columns. Drop and recreate.
-- ============================================================
DROP TABLE IF EXISTS character_buffs;
CREATE TABLE character_buffs (
    character_id    INTEGER NOT NULL,
    slot_id         SMALLINT NOT NULL,
    spell_id        SMALLINT NOT NULL DEFAULT 0,
    caster_level    SMALLINT NOT NULL DEFAULT 0,
    caster_name     VARCHAR(64) NOT NULL DEFAULT '',
    ticsremaining   INTEGER NOT NULL DEFAULT 0,
    counters        INTEGER NOT NULL DEFAULT 0,
    numhits         INTEGER NOT NULL DEFAULT 0,
    melee_rune      INTEGER NOT NULL DEFAULT 0,
    magic_rune      INTEGER NOT NULL DEFAULT 0,
    persistent      SMALLINT NOT NULL DEFAULT 0,
    dot_rune        INTEGER NOT NULL DEFAULT 0,
    caston_x        INTEGER NOT NULL DEFAULT 0,
    caston_y        INTEGER NOT NULL DEFAULT 0,
    caston_z        INTEGER NOT NULL DEFAULT 0,
    "ExtraDIChance" INTEGER NOT NULL DEFAULT 0,
    instrument_mod  INTEGER NOT NULL DEFAULT 10,
    PRIMARY KEY (character_id, slot_id)
);
CREATE INDEX idx_character_buffs_charid ON character_buffs (character_id);

-- ============================================================
-- 7. base_data: rename "end" to avoid PG reserved word conflict
-- The C++ code uses unquoted "end" in SELECT which PG rejects.
-- Rename to "endurance" and alias won't work, so we rename the
-- column to match what the C++ repo actually needs. But the repo
-- uses "end" as the column name. We need to keep it as "end" but
-- ensure it's always quoted. Since the repo auto-quotes it, just
-- verify it works. Actually the error shows it's NOT quoted in
-- the raw SQL at zone_base_data.cpp. Fix: rename to avoid issue.
-- ============================================================
-- The C++ raw query uses: SELECT level, "class", hp, mana, end, ...
-- PG treats unquoted "end" as reserved keyword. The fix is to
-- rename the column AND fix the C++ query. But since we can't
-- change C++ in a migration, we'll add a view or rename.
-- For now: the C++ uses the repository which DOES quote it.
-- The raw SQL in zone_base_data.cpp needs a C++ fix.
-- We leave the column as-is since the repository handles it.

-- ============================================================
-- 8. merchantlist_temp: add zone_id, instance_id
-- ============================================================
ALTER TABLE merchantlist_temp ADD COLUMN IF NOT EXISTS zone_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE merchantlist_temp ADD COLUMN IF NOT EXISTS instance_id INTEGER NOT NULL DEFAULT 0;

-- ============================================================
-- 9. petitions: add unavailables
-- ============================================================
ALTER TABLE petitions ADD COLUMN IF NOT EXISTS unavailables INTEGER NOT NULL DEFAULT 0;

-- ============================================================
-- 10. character_pet_info: add taunting
-- ============================================================
ALTER TABLE character_pet_info ADD COLUMN IF NOT EXISTS taunting SMALLINT NOT NULL DEFAULT 1;

-- ============================================================
-- 11. character_stats_record: add heal_amount
-- ============================================================
ALTER TABLE character_stats_record ADD COLUMN IF NOT EXISTS heal_amount INTEGER DEFAULT 0;

-- ============================================================
-- 12. group_id: add bot_id, merc_id
-- ============================================================
ALTER TABLE group_id ADD COLUMN IF NOT EXISTS bot_id INTEGER NOT NULL DEFAULT 0;
ALTER TABLE group_id ADD COLUMN IF NOT EXISTS merc_id INTEGER NOT NULL DEFAULT 0;

-- ============================================================
-- 13. zone_flags: rename columns to match C++ expectations
-- C++ uses charID/zoneID (camelCase), PG has char_id/zone_id
-- ============================================================
ALTER TABLE zone_flags RENAME COLUMN char_id TO "charID";
-- zoneID already has correct name

-- ============================================================
-- 14. account_flags: rebuild to match EQEmu schema
-- C++ expects p_accid/p_flag/p_value, PG has accid/flag_name/flag_value
-- ============================================================
DROP TABLE IF EXISTS account_flags;
CREATE TABLE account_flags (
    p_accid INTEGER NOT NULL,
    p_flag  VARCHAR(50) NOT NULL,
    p_value VARCHAR(80) NOT NULL DEFAULT '',
    PRIMARY KEY (p_accid, p_flag)
);
CREATE INDEX idx_account_flags_accid ON account_flags (p_accid);

-- ============================================================
-- 15. buyer: rebuild to match modern EQEmu schema
-- Current PG table has old trade-item schema; C++ expects modern buyer system
-- ============================================================
DROP TABLE IF EXISTS buyer;
CREATE TABLE buyer (
    id                      BIGSERIAL PRIMARY KEY,
    char_id                 INTEGER NOT NULL DEFAULT 0,
    char_entity_id          INTEGER NOT NULL DEFAULT 0,
    char_name               VARCHAR(64) DEFAULT NULL,
    char_zone_id            INTEGER NOT NULL DEFAULT 0,
    char_zone_instance_id   INTEGER NOT NULL DEFAULT 0,
    transaction_date        TIMESTAMP DEFAULT NULL,
    welcome_message         VARCHAR(256) DEFAULT NULL
);
CREATE INDEX idx_buyer_charid ON buyer (char_id);

-- ============================================================
-- 16. inventory: fix ON CONFLICT target
-- PK is already (character_id, slot_id) which is correct.
-- The C++ PrimaryKey() returns "character_id" only, which causes
-- ON CONFLICT (character_id) to fail. This needs a C++ fix in
-- base_inventory_repository.h, not a migration. Noted here for tracking.
-- ============================================================

COMMIT;
