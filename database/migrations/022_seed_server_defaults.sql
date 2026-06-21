-- 022_seed_server_defaults.sql
-- Default data required for server startup.
--
-- Without these rows, the server will crash or behave incorrectly on boot.
-- This is the minimum viable seed data — game content (items, NPCs, zones)
-- is seeded separately.

-- Default rule set (server loads ruleset_id=1 at boot)
INSERT INTO rule_sets (ruleset_id, name) VALUES (1, 'default')
    ON CONFLICT (ruleset_id) DO NOTHING;

-- Essential server rules (subset — add more as needed)
INSERT INTO rule_values (ruleset_id, rule_name, rule_value, notes) VALUES
    (1, 'World:MaxClients',       '100',   'Maximum simultaneous clients'),
    (1, 'World:AutoAccountCreate', 'true',  'Auto-create accounts on first login'),
    (1, 'World:MaxLevelLimit',    '65',     'Maximum player level'),
    (1, 'Zone:EnableShadowrest',  'false',  'Shadowrest corpse system'),
    (1, 'Zone:UseOldBindWound',   'false',  'Legacy bind wound behavior'),
    (1, 'Character:MaxLevel',     '65',     'Hard max level cap'),
    (1, 'Character:DeathExpLossLevel', '10', 'Level at which death causes exp loss'),
    (1, 'Character:CorpseDecayTimeMS', '10800000', 'Corpse decay time (3 hours)'),
    (1, 'Character:BindAnywhere', 'false',  'Allow binding in any zone'),
    (1, 'Combat:MeleeBaseCritChance', '0.0', 'Base melee crit chance'),
    (1, 'Combat:ClientBaseCritChance', '0.0', 'Client base crit chance')
ON CONFLICT (ruleset_id, rule_name) DO NOTHING;

-- Essential server variables
INSERT INTO variables (varname, value, information) VALUES
    ('DBVERSION',    '1',     'Database schema version'),
    ('MOTD',         'Welcome to ADIF - Another Day In Forever', 'Message of the Day')
ON CONFLICT (varname) DO NOTHING;

-- Logging categories (IDs must match eqemu_logsys.h enum values)
-- Start with all logging disabled; enable per-category as needed
INSERT INTO logsys_categories (log_category_id, log_category_description) VALUES
    (1,  'Zone'),
    (2,  'World'),
    (3,  'Login'),
    (4,  'UCS'),
    (5,  'Combat'),
    (6,  'Spells'),
    (7,  'Spawns'),
    (8,  'Guilds'),
    (9,  'Inventory'),
    (10, 'Trading'),
    (11, 'Pathing'),
    (12, 'Quests'),
    (13, 'Commands'),
    (14, 'Merchants'),
    (15, 'Crash')
ON CONFLICT (log_category_id) DO NOTHING;
