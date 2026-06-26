-- 037_fix_color_column_overflow.sql
-- Fix unsigned 32-bit color values overflowing PostgreSQL's signed INTEGER.
--
-- EQEmu uses uint32_t for ARGB color values (e.g. 0xFF000000 = 4278190080).
-- PostgreSQL INTEGER is signed 32-bit (max 2,147,483,647). Colors with high
-- alpha bytes overflow on INSERT. BIGINT (signed 64-bit) safely holds all
-- uint32_t values.

BEGIN;

-- inventory: color used for item tint
ALTER TABLE inventory ALTER COLUMN color TYPE BIGINT;

-- inventory_snapshots: mirrors inventory schema
ALTER TABLE inventory_snapshots ALTER COLUMN color TYPE BIGINT;

-- items: base item color
ALTER TABLE items ALTER COLUMN color TYPE BIGINT;

-- character_material: equipped item tint per slot
ALTER TABLE character_material ALTER COLUMN color TYPE BIGINT;

-- npc_types: NPC appearance colors (all uint32_t in C++)
ALTER TABLE npc_types ALTER COLUMN luclin_haircolor TYPE BIGINT;
ALTER TABLE npc_types ALTER COLUMN luclin_eyecolor TYPE BIGINT;
ALTER TABLE npc_types ALTER COLUMN luclin_eyecolor2 TYPE BIGINT;
ALTER TABLE npc_types ALTER COLUMN luclin_beardcolor TYPE BIGINT;

COMMIT;
