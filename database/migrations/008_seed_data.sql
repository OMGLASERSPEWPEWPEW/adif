-- Seed data for testing: two zones, some NPCs, and Ghouldan.

-- ── Zones ──────────────────────────────────────────────────────

INSERT INTO zones (short_name, long_name, safe_x, safe_y, safe_z, geometry_type, can_bind, exp_multiplier)
VALUES
    ('innothule', 'Innothule Swamp', -500, 200, -25, 'voxel', FALSE, 1.0),
    ('grobb',     'Grobb',           0,   0,   5,   'mesh',  TRUE,  0.8);

-- Zone transition: Innothule <-> Grobb
INSERT INTO zone_points (zone_id, number, x, y, z, heading, width, height, target_zone_id, target_x, target_y, target_z, target_heading)
VALUES
    (1, 1, 1200, -400, 0, 0, 10, 15, 2, -50, 100, 5, 180),
    (2, 1, -50, 100, 5, 180, 10, 15, 1, 1200, -400, 0, 0);

-- ── Account & Character ────────────────────────────────────────

INSERT INTO accounts (name, password_hash, status)
VALUES ('ghouldan_account', 'placeholder_hash', 250);

INSERT INTO characters (
    account_id, name, race, class_id, level, gender, deity,
    str, sta, dex, agi, int_, wis, cha,
    current_hp, max_hp, current_mana, max_mana,
    zone_id, x, y, z, heading,
    platinum, is_gm,
    appearance
) VALUES (
    1, 'Ghouldan', 12, 5, 65, 0, 0,   -- ogre shadow knight
    200, 200, 200, 200, 200, 200, 200, -- maxed stats
    32000, 32000, 8000, 8000,          -- big HP/mana pool
    1, -500, 200, -25, 0,             -- spawn in Innothule
    99999, TRUE,                       -- rich and GM
    '{"hair_color": 0, "beard_color": 0, "face": 3, "hair_style": 0}'
);

-- ── NPCs ───────────────────────────────────────────────────────

INSERT INTO npc_templates (name, level, race, class_id, hp, min_damage, max_damage, aggro_radius, run_speed, loot_table_id)
VALUES
    ('a_froglok_warrior',  15, 74, 1, 750,  10, 35, 70, 1.25, NULL),
    ('a_froglok_shaman',   18, 74, 10, 600, 8,  25, 70, 1.0,  NULL),
    ('a_large_mosquito',   8,  75, 1, 200,  5,  15, 50, 1.5,  NULL),
    ('a_swamp_alligator',  12, 76, 1, 500,  12, 30, 60, 0.8,  NULL),
    ('Grobb_Guard',        35, 12, 1, 3500, 30, 85, 100, 1.5, NULL);

-- Spawn groups
INSERT INTO spawn_groups (name) VALUES
    ('innothule_frogloks'),
    ('innothule_wildlife'),
    ('grobb_guards');

-- Spawn entries (which NPCs in which groups)
INSERT INTO spawn_entries (spawn_group_id, npc_template_id, chance) VALUES
    (1, 1, 70),   -- 70% froglok warrior
    (1, 2, 30),   -- 30% froglok shaman
    (2, 3, 60),   -- 60% mosquito
    (2, 4, 40),   -- 40% alligator
    (3, 5, 100);  -- 100% Grobb Guard

-- Spawn points in Innothule
INSERT INTO spawn_points (zone_id, spawn_group_id, x, y, z, heading, respawn_time) VALUES
    (1, 1, -200, 100, -25, 90,  300),
    (1, 1, -300, 50,  -25, 180, 300),
    (1, 1, -100, 250, -25, 0,   300),
    (1, 2, -400, 300, -25, 45,  180),
    (1, 2, -600, 150, -25, 270, 180),
    (1, 2, -150, 400, -25, 135, 180);

-- Spawn points in Grobb
INSERT INTO spawn_points (zone_id, spawn_group_id, x, y, z, heading, respawn_time) VALUES
    (2, 3, 50,  10, 5, 0,   600),
    (2, 3, -50, -10, 5, 180, 600);

-- ── Patrol Grid (froglok patrol in Innothule) ──────────────────

INSERT INTO patrol_grids (zone_id, wander_type, pause_type) VALUES (1, 2, 1);

INSERT INTO patrol_waypoints (grid_id, number, x, y, z, heading, pause_ms) VALUES
    (1, 1, -200, 100, -25, 0,   5000),
    (1, 2, -250, 150, -25, 45,  3000),
    (1, 3, -300, 200, -25, 90,  5000),
    (1, 4, -250, 150, -25, 225, 3000);

-- Link first spawn point to this patrol grid
UPDATE spawn_points SET patrol_grid_id = 1 WHERE id = 1;
