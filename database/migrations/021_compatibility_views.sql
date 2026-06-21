-- 021_compatibility_views.sql
-- PostgreSQL VIEWs mapping EQEmu table names to ADIF canonical names.
--
-- The EQServer C++ code has duplicate repository classes that reference both
-- old EQEmu names and new ADIF names. These views let both work against the
-- same underlying data without duplication.
--
-- Simple views (column-compatible) are directly updatable in PostgreSQL.
-- Views with column renames need INSTEAD OF triggers for write operations.

-- Zone alias: zone → zones
CREATE OR REPLACE VIEW zone AS SELECT * FROM zones;

-- Account alias: account → accounts
CREATE OR REPLACE VIEW account AS SELECT * FROM accounts;

-- Spawn aliases
CREATE OR REPLACE VIEW spawn2 AS SELECT * FROM spawn_points;
CREATE OR REPLACE VIEW spawngroup AS SELECT * FROM spawn_groups;
CREATE OR REPLACE VIEW spawnentry AS SELECT * FROM spawn_entries;

-- Loot aliases
CREATE OR REPLACE VIEW loottable AS SELECT * FROM loot_tables;
CREATE OR REPLACE VIEW lootdrop AS SELECT * FROM loot_drops;
CREATE OR REPLACE VIEW loottable_entries AS SELECT * FROM loot_table_entries;
CREATE OR REPLACE VIEW lootdrop_entries AS SELECT * FROM loot_drop_entries;

-- Navigation aliases
CREATE OR REPLACE VIEW grid AS SELECT * FROM patrol_grids;
CREATE OR REPLACE VIEW grid_entries AS SELECT * FROM patrol_waypoints;

-- Merchant alias
CREATE OR REPLACE VIEW merchantlist AS SELECT * FROM merchant_lists;

-- NPC spell aliases
CREATE OR REPLACE VIEW npc_spells AS SELECT * FROM npc_spell_lists;
CREATE OR REPLACE VIEW npc_spells_entries AS SELECT * FROM npc_spell_entries;

-- Spell alias: spells_new → spells
CREATE OR REPLACE VIEW spells_new AS SELECT * FROM spells;

-- NPC types alias: npc_types → npc_templates
-- The ADIF npc_templates table uses JSONB for stats/resistances/appearance/
-- special_abilities. The C++ npc_types repo expects flat columns. This view
-- extracts JSONB fields so the C++ code can read them.
-- For writes, an INSTEAD OF trigger handles JSONB packing.
CREATE OR REPLACE VIEW npc_types AS
SELECT
    id,
    name,
    level,
    race,
    class_id AS "class",
    (stats->>'hp')::INTEGER AS hp,
    (stats->>'mana')::INTEGER AS mana,
    gender,
    texture,
    helmtexture,
    size,
    (stats->>'hp_regen_rate')::INTEGER AS hp_regen_rate,
    (stats->>'mana_regen_rate')::INTEGER AS mana_regen_rate,
    loottable_id,
    merchant_id,
    npc_spells_id,
    npc_faction_id,
    (stats->>'mindmg')::INTEGER AS mindmg,
    (stats->>'maxdmg')::INTEGER AS maxdmg,
    (stats->>'attack_speed')::SMALLINT AS attack_speed,
    (stats->>'aggroradius')::INTEGER AS aggroradius,
    bodytype,
    (stats->>'ac')::SMALLINT AS ac,
    (stats->>'str')::SMALLINT AS str,
    (stats->>'sta')::SMALLINT AS sta,
    (stats->>'dex')::SMALLINT AS dex,
    (stats->>'agi')::SMALLINT AS agi,
    (stats->>'int')::SMALLINT AS _int,
    (stats->>'wis')::SMALLINT AS wis,
    (stats->>'cha')::SMALLINT AS cha,
    (resistances->>'mr')::SMALLINT AS mr,
    (resistances->>'cr')::SMALLINT AS cr,
    (resistances->>'dr')::SMALLINT AS dr,
    (resistances->>'fr')::SMALLINT AS fr,
    (resistances->>'pr')::SMALLINT AS pr,
    (appearance->>'face')::INTEGER AS face,
    (appearance->>'luclin_hairstyle')::INTEGER AS luclin_hairstyle,
    (appearance->>'luclin_haircolor')::INTEGER AS luclin_haircolor,
    (appearance->>'luclin_eyecolor')::INTEGER AS luclin_eyecolor,
    (appearance->>'luclin_eyecolor2')::INTEGER AS luclin_eyecolor2,
    (appearance->>'luclin_beardcolor')::INTEGER AS luclin_beardcolor,
    (appearance->>'luclin_beard')::INTEGER AS luclin_beard,
    runspeed,
    special_abilities::TEXT AS special_abilities
FROM npc_templates;

-- Character data alias: character_data → characters
-- The ADIF characters table uses JSONB for appearance and bind_points.
-- This view exposes them as flat columns for the C++ code.
CREATE OR REPLACE VIEW character_data AS
SELECT
    id,
    account_id,
    name,
    level,
    race,
    class_id AS "class",
    gender,
    deity,
    zone_id,
    x, y, z, heading,
    hp, mana, endurance,
    experience AS exp,
    platinum, gold, silver, copper,
    (appearance->>'face')::SMALLINT AS face,
    (appearance->>'hair_color')::SMALLINT AS hair_color,
    (appearance->>'hair_style')::SMALLINT AS hair_style,
    (appearance->>'beard')::SMALLINT AS beard,
    (appearance->>'beard_color')::SMALLINT AS beard_color,
    (appearance->>'eye_color_1')::SMALLINT AS eye_color_1,
    (appearance->>'eye_color_2')::SMALLINT AS eye_color_2,
    (appearance->>'texture')::SMALLINT AS texture,
    (appearance->>'helm_texture')::SMALLINT AS helm_texture,
    gm_status AS status,
    created_at,
    updated_at
FROM characters;
