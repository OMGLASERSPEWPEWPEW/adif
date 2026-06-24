-- 034_remaining_fixes.sql
-- Fix remaining errors blocking zone entry and reducing startup noise.

BEGIN;

-- ============================================================
-- group_id: rename charid BACK to character_id (C++ uses character_id)
-- Migration 033 renamed it the wrong direction.
-- ============================================================
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='group_id' AND column_name='charid')
    AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='group_id' AND column_name='character_id')
    THEN ALTER TABLE group_id RENAME COLUMN charid TO character_id;
    END IF;
END $$;

-- ============================================================
-- respawn_times: add missing instance_id and fix PK
-- ============================================================
ALTER TABLE respawn_times ADD COLUMN IF NOT EXISTS instance_id SMALLINT NOT NULL DEFAULT 0;
ALTER TABLE respawn_times DROP CONSTRAINT IF EXISTS respawn_times_pkey;
ALTER TABLE respawn_times ADD PRIMARY KEY (id, instance_id);

-- ============================================================
-- Tier 2 missing tables — reduce startup error noise
-- ============================================================

CREATE TABLE IF NOT EXISTS raid_leaders (
    gid INTEGER NOT NULL DEFAULT 0,
    rid INTEGER NOT NULL DEFAULT 0,
    marknpc TEXT NOT NULL DEFAULT '',
    maintank TEXT NOT NULL DEFAULT '',
    assist TEXT NOT NULL DEFAULT '',
    puller TEXT NOT NULL DEFAULT '',
    leadershipaa BYTEA,
    PRIMARY KEY (gid)
);

CREATE TABLE IF NOT EXISTS inventory_snapshots (
    time_index INTEGER NOT NULL DEFAULT 0,
    charid INTEGER NOT NULL DEFAULT 0,
    slotid INTEGER NOT NULL DEFAULT 0,
    itemid INTEGER NOT NULL DEFAULT 0,
    charges SMALLINT NOT NULL DEFAULT 0,
    color INTEGER NOT NULL DEFAULT 0,
    augslot1 INTEGER NOT NULL DEFAULT 0,
    augslot2 INTEGER NOT NULL DEFAULT 0,
    augslot3 INTEGER NOT NULL DEFAULT 0,
    augslot4 INTEGER NOT NULL DEFAULT 0,
    augslot5 INTEGER NOT NULL DEFAULT 0,
    augslot6 INTEGER NOT NULL DEFAULT 0,
    instnodrop SMALLINT NOT NULL DEFAULT 0,
    custom_data TEXT NOT NULL DEFAULT '',
    ornamenticon INTEGER NOT NULL DEFAULT 0,
    ornamentidfile INTEGER NOT NULL DEFAULT 0,
    ornament_hero_model INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (time_index, charid, slotid)
);

CREATE TABLE IF NOT EXISTS buyer (
    charid INTEGER NOT NULL DEFAULT 0 PRIMARY KEY,
    buyslot INTEGER NOT NULL DEFAULT 0,
    itemid INTEGER NOT NULL DEFAULT 0,
    itemname TEXT NOT NULL DEFAULT '',
    quantity INTEGER NOT NULL DEFAULT 0,
    price INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS buyer_buy_lines (
    id SERIAL PRIMARY KEY,
    buyer_id INTEGER NOT NULL DEFAULT 0,
    buy_slot INTEGER NOT NULL DEFAULT 0,
    item_id INTEGER NOT NULL DEFAULT 0,
    item_name TEXT NOT NULL DEFAULT '',
    item_quantity INTEGER NOT NULL DEFAULT 0,
    item_price INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS buyer_trade_items (
    id SERIAL PRIMARY KEY,
    buyer_id INTEGER NOT NULL DEFAULT 0,
    buy_slot INTEGER NOT NULL DEFAULT 0,
    item_id INTEGER NOT NULL DEFAULT 0,
    item_quantity INTEGER NOT NULL DEFAULT 0,
    item_icon INTEGER NOT NULL DEFAULT 0,
    item_name TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS guild_permissions (
    id SERIAL PRIMARY KEY,
    perm_id INTEGER NOT NULL DEFAULT 0,
    guild_id INTEGER NOT NULL DEFAULT 0,
    permission INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS guild_tributes (
    guild_id INTEGER NOT NULL DEFAULT 0 PRIMARY KEY,
    tribute_id_1 INTEGER NOT NULL DEFAULT 0,
    tribute_id_1_tier INTEGER NOT NULL DEFAULT 0,
    tribute_id_2 INTEGER NOT NULL DEFAULT 0,
    tribute_id_2_tier INTEGER NOT NULL DEFAULT 0,
    time_remaining INTEGER NOT NULL DEFAULT 0,
    enabled SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS tributes (
    id INTEGER NOT NULL PRIMARY KEY,
    unknown INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL DEFAULT '',
    descr TEXT NOT NULL DEFAULT '',
    isguild SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS tribute_levels (
    tribute_id INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 0,
    cost INTEGER NOT NULL DEFAULT 0,
    item_id INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (tribute_id, level)
);

CREATE TABLE IF NOT EXISTS character_expedition_lockouts (
    id SERIAL PRIMARY KEY,
    character_id INTEGER NOT NULL DEFAULT 0,
    expedition_name TEXT NOT NULL DEFAULT '',
    event_name TEXT NOT NULL DEFAULT '',
    expire_time TIMESTAMP NOT NULL DEFAULT NOW(),
    duration INTEGER NOT NULL DEFAULT 0,
    from_expedition_uuid TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS character_task_timers (
    id SERIAL PRIMARY KEY,
    character_id INTEGER NOT NULL DEFAULT 0,
    task_id INTEGER NOT NULL DEFAULT 0,
    timer_type INTEGER NOT NULL DEFAULT 0,
    timer_group INTEGER NOT NULL DEFAULT 0,
    expire_time TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER NOT NULL PRIMARY KEY,
    type SMALLINT NOT NULL DEFAULT 0,
    duration INTEGER NOT NULL DEFAULT 0,
    duration_code SMALLINT NOT NULL DEFAULT 0,
    title TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    reward_text TEXT NOT NULL DEFAULT '',
    reward_id_list TEXT NOT NULL DEFAULT '',
    cash_reward INTEGER NOT NULL DEFAULT 0,
    exp_reward INTEGER NOT NULL DEFAULT 0,
    reward_method SMALLINT NOT NULL DEFAULT 0,
    reward_points INTEGER NOT NULL DEFAULT 0,
    reward_point_type INTEGER NOT NULL DEFAULT 0,
    min_level SMALLINT NOT NULL DEFAULT 0,
    max_level SMALLINT NOT NULL DEFAULT 0,
    level_spread INTEGER NOT NULL DEFAULT 0,
    min_players INTEGER NOT NULL DEFAULT 0,
    max_players INTEGER NOT NULL DEFAULT 0,
    repeatable SMALLINT NOT NULL DEFAULT 0,
    faction_reward INTEGER NOT NULL DEFAULT 0,
    completion_emote TEXT NOT NULL DEFAULT '',
    replay_timer_group INTEGER NOT NULL DEFAULT 0,
    replay_timer_seconds INTEGER NOT NULL DEFAULT 0,
    request_timer_group INTEGER NOT NULL DEFAULT 0,
    request_timer_seconds INTEGER NOT NULL DEFAULT 0,
    dz_template_id INTEGER NOT NULL DEFAULT 0,
    lock_activity_id INTEGER NOT NULL DEFAULT -1,
    faction_amount INTEGER NOT NULL DEFAULT 0,
    enabled SMALLINT NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS task_activities (
    taskid INTEGER NOT NULL DEFAULT 0,
    activityid SMALLINT NOT NULL DEFAULT 0,
    req_activity_id INTEGER NOT NULL DEFAULT -1,
    step INTEGER NOT NULL DEFAULT 0,
    activitytype SMALLINT NOT NULL DEFAULT 0,
    target_name TEXT NOT NULL DEFAULT '',
    goalmethod INTEGER NOT NULL DEFAULT 0,
    goalcount INTEGER NOT NULL DEFAULT 1,
    description_override TEXT NOT NULL DEFAULT '',
    npc_match_list TEXT NOT NULL DEFAULT '',
    item_id_list TEXT NOT NULL DEFAULT '',
    item_list TEXT NOT NULL DEFAULT '',
    dz_switch_id INTEGER NOT NULL DEFAULT 0,
    min_x REAL NOT NULL DEFAULT 0,
    min_y REAL NOT NULL DEFAULT 0,
    min_z REAL NOT NULL DEFAULT 0,
    max_x REAL NOT NULL DEFAULT 0,
    max_y REAL NOT NULL DEFAULT 0,
    max_z REAL NOT NULL DEFAULT 0,
    skill_list TEXT NOT NULL DEFAULT '-1',
    spell_list TEXT NOT NULL DEFAULT '0',
    zones TEXT NOT NULL DEFAULT '',
    zone_version INTEGER NOT NULL DEFAULT -1,
    optional SMALLINT NOT NULL DEFAULT 0,
    list_group INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (taskid, activityid)
);

CREATE TABLE IF NOT EXISTS tasksets (
    id INTEGER NOT NULL DEFAULT 0,
    taskid INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id, taskid)
);

CREATE TABLE IF NOT EXISTS shared_tasks (
    id BIGSERIAL PRIMARY KEY,
    task_id INTEGER NOT NULL DEFAULT 0,
    accepted_time TIMESTAMP,
    expire_time TIMESTAMP,
    completion_time TIMESTAMP,
    is_locked SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS shared_task_members (
    shared_task_id BIGINT NOT NULL DEFAULT 0,
    character_id BIGINT NOT NULL DEFAULT 0,
    is_leader SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (shared_task_id, character_id)
);

CREATE TABLE IF NOT EXISTS shared_task_activity_state (
    shared_task_id BIGINT NOT NULL DEFAULT 0,
    activity_id INTEGER NOT NULL DEFAULT 0,
    done_count INTEGER NOT NULL DEFAULT 0,
    updated_time TIMESTAMP,
    completed_time TIMESTAMP,
    PRIMARY KEY (shared_task_id, activity_id)
);

CREATE TABLE IF NOT EXISTS shared_task_dynamic_zones (
    shared_task_id BIGINT NOT NULL DEFAULT 0,
    dynamic_zone_id INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (shared_task_id, dynamic_zone_id)
);

CREATE TABLE IF NOT EXISTS dynamic_zones (
    id SERIAL PRIMARY KEY,
    uuid TEXT NOT NULL DEFAULT '',
    name TEXT NOT NULL DEFAULT '',
    leader_id INTEGER NOT NULL DEFAULT 0,
    min_players INTEGER NOT NULL DEFAULT 0,
    max_players INTEGER NOT NULL DEFAULT 0,
    instance_id INTEGER NOT NULL DEFAULT 0,
    type SMALLINT NOT NULL DEFAULT 0,
    dz_switch_id INTEGER NOT NULL DEFAULT 0,
    compass_zone_id INTEGER NOT NULL DEFAULT 0,
    compass_x REAL NOT NULL DEFAULT 0,
    compass_y REAL NOT NULL DEFAULT 0,
    compass_z REAL NOT NULL DEFAULT 0,
    safe_return_zone_id INTEGER NOT NULL DEFAULT 0,
    safe_return_x REAL NOT NULL DEFAULT 0,
    safe_return_y REAL NOT NULL DEFAULT 0,
    safe_return_z REAL NOT NULL DEFAULT 0,
    safe_return_heading REAL NOT NULL DEFAULT 0,
    zone_in_x REAL NOT NULL DEFAULT 0,
    zone_in_y REAL NOT NULL DEFAULT 0,
    zone_in_z REAL NOT NULL DEFAULT 0,
    zone_in_heading REAL NOT NULL DEFAULT 0,
    has_zone_in SMALLINT NOT NULL DEFAULT 0,
    is_locked SMALLINT NOT NULL DEFAULT 0,
    add_replay SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS dynamic_zone_members (
    id SERIAL PRIMARY KEY,
    dynamic_zone_id INTEGER NOT NULL DEFAULT 0,
    character_id INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS dynamic_zone_lockouts (
    id SERIAL PRIMARY KEY,
    dynamic_zone_id INTEGER NOT NULL DEFAULT 0,
    event_name TEXT NOT NULL DEFAULT '',
    expire_time TIMESTAMP,
    duration INTEGER NOT NULL DEFAULT 0,
    from_expedition_uuid TEXT NOT NULL DEFAULT ''
);

CREATE TABLE IF NOT EXISTS dynamic_zone_templates (
    id SERIAL PRIMARY KEY,
    zone_id INTEGER NOT NULL DEFAULT 0,
    zone_version INTEGER NOT NULL DEFAULT 0,
    name TEXT NOT NULL DEFAULT '',
    min_players INTEGER NOT NULL DEFAULT 0,
    max_players INTEGER NOT NULL DEFAULT 0,
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    dz_switch_id INTEGER NOT NULL DEFAULT 0,
    compass_zone_id INTEGER NOT NULL DEFAULT 0,
    compass_x REAL NOT NULL DEFAULT 0,
    compass_y REAL NOT NULL DEFAULT 0,
    compass_z REAL NOT NULL DEFAULT 0,
    return_zone_id INTEGER NOT NULL DEFAULT 0,
    return_x REAL NOT NULL DEFAULT 0,
    return_y REAL NOT NULL DEFAULT 0,
    return_z REAL NOT NULL DEFAULT 0,
    return_h REAL NOT NULL DEFAULT 0,
    override_zone_in SMALLINT NOT NULL DEFAULT 0,
    zone_in_x REAL NOT NULL DEFAULT 0,
    zone_in_y REAL NOT NULL DEFAULT 0,
    zone_in_z REAL NOT NULL DEFAULT 0,
    zone_in_h REAL NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS zone_state_spawns (
    id SERIAL PRIMARY KEY,
    zone_id INTEGER NOT NULL DEFAULT 0,
    instance_id INTEGER NOT NULL DEFAULT 0,
    is_corpse SMALLINT NOT NULL DEFAULT 0,
    is_zone SMALLINT NOT NULL DEFAULT 0,
    decay_in_seconds INTEGER NOT NULL DEFAULT 0,
    npc_id INTEGER NOT NULL DEFAULT 0,
    spawn2_id INTEGER NOT NULL DEFAULT 0,
    spawngroup_id INTEGER NOT NULL DEFAULT 0,
    x REAL NOT NULL DEFAULT 0,
    y REAL NOT NULL DEFAULT 0,
    z REAL NOT NULL DEFAULT 0,
    heading REAL NOT NULL DEFAULT 0,
    respawn_time INTEGER NOT NULL DEFAULT 0,
    variance INTEGER NOT NULL DEFAULT 0,
    grid INTEGER NOT NULL DEFAULT 0,
    current_waypoint INTEGER NOT NULL DEFAULT 0,
    path_when_zone_idle SMALLINT NOT NULL DEFAULT 0,
    condition_id INTEGER NOT NULL DEFAULT 0,
    condition_min_value INTEGER NOT NULL DEFAULT 0,
    enabled SMALLINT NOT NULL DEFAULT 1,
    anim SMALLINT NOT NULL DEFAULT 0,
    loot_data TEXT,
    entity_variables TEXT,
    buffs TEXT,
    hp INTEGER NOT NULL DEFAULT 0,
    mana INTEGER NOT NULL DEFAULT 0,
    endurance INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS npc_scale_global_base (
    type INTEGER NOT NULL DEFAULT 0,
    level INTEGER NOT NULL DEFAULT 0,
    zone_id_list TEXT NOT NULL DEFAULT '',
    instance_version_list TEXT NOT NULL DEFAULT '',
    ac INTEGER NOT NULL DEFAULT 0,
    hp BIGINT NOT NULL DEFAULT 0,
    accuracy INTEGER NOT NULL DEFAULT 0,
    slow_mitigation INTEGER NOT NULL DEFAULT 0,
    attack INTEGER NOT NULL DEFAULT 0,
    strength INTEGER NOT NULL DEFAULT 0,
    stamina INTEGER NOT NULL DEFAULT 0,
    dexterity INTEGER NOT NULL DEFAULT 0,
    agility INTEGER NOT NULL DEFAULT 0,
    intelligence INTEGER NOT NULL DEFAULT 0,
    wisdom INTEGER NOT NULL DEFAULT 0,
    charisma INTEGER NOT NULL DEFAULT 0,
    magic_resist INTEGER NOT NULL DEFAULT 0,
    cold_resist INTEGER NOT NULL DEFAULT 0,
    fire_resist INTEGER NOT NULL DEFAULT 0,
    poison_resist INTEGER NOT NULL DEFAULT 0,
    disease_resist INTEGER NOT NULL DEFAULT 0,
    corruption_resist INTEGER NOT NULL DEFAULT 0,
    physical_resist INTEGER NOT NULL DEFAULT 0,
    min_dmg INTEGER NOT NULL DEFAULT 0,
    max_dmg INTEGER NOT NULL DEFAULT 0,
    hp_regen_rate BIGINT NOT NULL DEFAULT 0,
    hp_regen_per_second BIGINT NOT NULL DEFAULT 0,
    attack_delay INTEGER NOT NULL DEFAULT 0,
    spell_scale REAL NOT NULL DEFAULT 100,
    heal_scale REAL NOT NULL DEFAULT 100,
    avoidance INTEGER NOT NULL DEFAULT 0,
    heroic_strikethrough INTEGER NOT NULL DEFAULT 0,
    special_abilities TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (type, level)
);

CREATE TABLE IF NOT EXISTS items_evolving_details (
    id SERIAL PRIMARY KEY,
    item_evo_id INTEGER NOT NULL DEFAULT 0,
    item_evolve_level SMALLINT NOT NULL DEFAULT 0,
    item_id INTEGER NOT NULL DEFAULT 0,
    type SMALLINT NOT NULL DEFAULT 0,
    sub_type SMALLINT NOT NULL DEFAULT 0,
    required_amount INTEGER NOT NULL DEFAULT 0
);

-- ============================================================
-- Clean up stale character data
-- ============================================================
DELETE FROM character_data;
DELETE FROM character_bind;
DELETE FROM character_skills;
DELETE FROM character_languages;

COMMIT;
