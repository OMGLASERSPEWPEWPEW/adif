# PostgreSQL Migration — Error Inventory

Complete catalog of every PostgreSQL error observed in the EQEmu server logs
after MySQL→PostgreSQL conversion. Updated 2026-06-25.

---

## REMAINING ERRORS (updated 2026-06-25 — zone entry working)

Zone boots Grobb successfully, character enters zone. These errors are non-fatal
but affect gameplay features (inventory, spells, doors, etc.).

### A. Missing Tables (~22 tables)

Tables the C++ code queries that don't exist in PostgreSQL yet.

| # | Table | When | Query Context |
|---|-------|------|---------------|
| 1 | `character_item_recast` | Character load | DELETE/SELECT recast timers |
| 2 | `character_evolving_items` | Character load | SELECT evolving item progress |
| 3 | `sharedbank` | Character load | SELECT shared bank slots |
| 4 | `character_bandolier` | Character load | SELECT weapon sets |
| 5 | `character_potionbelt` | Character load | SELECT potion belt |
| 6 | `character_leadership_abilities` | Character load | SELECT leadership AAs |
| 7 | `character_tribute` | Character load | SELECT tribute settings |
| 8 | `character_exp_modifiers` | Character load | SELECT zone XP mods |
| 9 | `character_tasks` | Character load | SELECT active tasks |
| 10 | `character_activities` | Character load | SELECT task progress |
| 11 | `completed_tasks` | Character load | SELECT completed task history |
| 12 | `character_enabledtasks` | Character load | SELECT unlocked tasks |
| 13 | `character_disciplines` | Character load | SELECT learned disciplines |
| 14 | `character_auras` | Character load | SELECT active auras |
| 15 | `character_alt_currency` | Character load | SELECT alt currencies |
| 16 | `character_instance_safereturns` | Character load | DELETE safe return points |
| 17 | `character_peqzone_flags` | Character load | SELECT zone access flags |
| 18 | `completed_shared_task_members` | Character load | SELECT shared task history |
| 19 | `adventure_members` | Character load | SELECT adventure group |
| 20 | `keyring` | Character load | SELECT key items |
| 21 | `veteran_reward_templates` | Zone boot | SELECT veteran rewards |
| 22 | `adventure_template_entry_flavor` | Zone boot | SELECT adventure text |

### B. Missing/Wrong Columns (~12 issues)

Columns the C++ code references that are named differently or missing in PG.

| # | Table | Column Issue | C++ Expects | PG Has | Fix |
|---|-------|-------------|-------------|--------|-----|
| 1 | `character_corpses` | Missing column | `instance_id` | not present | ALTER TABLE ADD COLUMN |
| 2 | `object_contents` | Missing columns | `augslot1`-`augslot6` | not present | ALTER TABLE ADD COLUMN ×6 |
| 3 | `doors` | Missing column | `close_timer_ms` | not present | ALTER TABLE ADD COLUMN |
| 4 | `character_spells` | Wrong column name | `slot_id` | `slot` | ALTER TABLE RENAME COLUMN |
| 5 | `character_memmed_spells` | Wrong column name | `slot_id` | `slot` | ALTER TABLE RENAME COLUMN |
| 6 | `character_buffs` | Wrong column name | `character_id` | `id` | ALTER TABLE RENAME or adjust query |
| 7 | `base_data` | Reserved word | `end` | `end` (PG reserved) | Rename to `endurance` or quote as `"end"` |
| 8 | `merchantlist_temp` | Missing column | `zone_id` | not present | ALTER TABLE ADD COLUMN |
| 9 | `petitions` | Missing column | `unavailables` | not present | ALTER TABLE ADD COLUMN |
| 10 | `character_pet_info` | Missing column | `taunting` | not present | ALTER TABLE ADD COLUMN |
| 11 | `character_stats_record` | Missing column | `heal_amount` | not present | ALTER TABLE ADD COLUMN |
| 12 | `group_id` | Missing column | `bot_id` | not present | ALTER TABLE ADD COLUMN |

### C. Wrong Column Names in Non-Repository Queries

Raw SQL in C++ files that uses MySQL column names not matching PG schema.

| # | Table | C++ Query Column | PG Column | File |
|---|-------|-----------------|-----------|------|
| 1 | `zone_flags` | `charID`, `zoneID` | `char_id`, `zone_id` | zone/zonedb.cpp |
| 2 | `account_flags` | `p_accid`, `p_flag`, `p_value` | `accid`, `flag`, `value` | zone/zonedb.cpp |
| 3 | `raid_members` | `bot_id` | not present | zone/raids.cpp |
| 4 | `buyer` | `id` column | not present or named differently | zone/client.cpp |

### D. ON CONFLICT / Constraint Issues

| # | Table | Issue | Fix |
|---|-------|-------|-----|
| 1 | `inventory` | `ON CONFLICT (character_id)` but PK is composite | Fix PrimaryKey() or add unique constraint |

### E. World Startup (Cosmetic, Low Priority)

Same as before — MySQL DDL in `database_update_manifest.h`. Non-fatal log noise.

| Error | Query |
|-------|-------|
| `SHOW COLUMNS FROM db_version LIKE 'custom_version'` | MySQL introspection |
| `INT(11) UNSIGNED NOT NULL` | MySQL type syntax |
| `ALTER TABLE ... ADD ... AFTER column` | MySQL positional ALTER |
| `show columns from db_version where Field like '%bots_version%'` | MySQL introspection |

### F. C++ Code Fixes Applied This Session (Not Yet Committed)

| # | File | Fix | Description |
|---|------|-----|-------------|
| 1 | `world/zoneserver.cpp` | `zone_boot_timer(5000)` → `(30000)` | Increased boot timer for PG |
| 2 | `zone/zone.cpp:84-88` | `SetZoneData(0)` → `SetZoneData(zone->GetZoneID(), ...)` | Fix "already booted" state corruption |
| 3 | `common/repositories/criteria/content_filter_criteria.h` | `CONCAT`/`REGEXP` → `\|\|`/`~` | PostgreSQL string/regex syntax |
| 4 | `zone/quest_parser_collection.cpp:1072` | `FindReplace("", "-")` → `FindReplace(" ", "-")` | Fix infinite loop on empty string match |

---

## FIXED — Complete History

### Session 2026-06-21: Initial PostgreSQL Setup
| # | Error | Fix |
|---|-------|-----|
| 1 | MariaDB crash-loop (InnoDB redo log version) | Upgraded Dockerfile to mariadb:10.11 |
| 2 | login.json port type (string vs int) | Changed to integer 5433 |
| 3 | Account id=0 treated as "not found" | Updated to id=1, reset sequence |

### Session 2026-06-23: Character Creation + Zone Boot
| # | Error | Fix |
|---|-------|-----|
| 1 | `deleted_at` type mismatch (TIMESTAMP vs INTEGER) | Reverted to TIMESTAMP |
| 2 | `duplicate key (id)=(0)` on character_data | Added `auto_id_on_zero()` trigger |
| 3 | InsertOne returns id=0 → name rejection | Added `RETURNING id` to 250 base repos |
| 4 | `RestTimer` case mismatch | Renamed column to lowercase |
| 5 | `character_bind` missing `instance_id` | ALTER TABLE ADD COLUMN |
| 6 | `character_skills/spells/memmed_spells/buffs` wrong column name | Renamed `character_id` → `id` |
| 7 | `group_id` column mismatch | Renamed to match C++ expectations |
| 8 | `starting_items` missing 12 columns | ALTER TABLE ADD COLUMN (class_list, race_list, etc.) |
| 9 | Zone server exit (missing quest plugins) | Created check_handin stubs |
| 10 | Tutorial zone as start zone | Disabled via rules |
| 11 | 9 table name mismatches | Migration 030: table renames |
| 12 | Column mismatches on 8 tables | Migration 031: schema fixes |
| 13 | ~17 missing Tier 1 tables | Migration 032: AA, base_data, factions, etc. |
| 14 | 5 column name/missing column issues | Migration 033: starting_items, level_exp_mods, respawn_times, spawn_condition_values |
| 15 | ~26 missing Tier 2 tables | Migration 034: raids, buyer, guilds, tributes, DZ, tasks, etc. |
| 16 | 4 missing zone boot tables | Migration 033: spawn2_disabled, global_loot, ldon_trap_* |

### Session 2026-06-24: ON CONFLICT Fixes + MySQL Syntax Cleanup
| # | Error | Fix |
|---|-------|-----|
| 1 | `ON CONFLICT (id)` on character_bind (PK is id,slot) | Fixed PrimaryKey() → `"id, slot"` |
| 2 | `ON CONFLICT (id)` on character_skills (PK is id,skill_id) | Fixed PrimaryKey() → `"id, skill_id"` |
| 3 | `ON CONFLICT (id)` on character_languages (PK is id,lang_id) | Fixed PrimaryKey() → `"id, lang_id"` |
| 4 | `ON CONFLICT (ruleset_id)` on rule_values (PK is ruleset_id,rule_name) | Fixed PrimaryKey() → `"ruleset_id, rule_name"` |
| 5 | `UPDATE eqtime ... LIMIT 1` | Removed LIMIT (common/database.cpp) |
| 6 | `time_of_death != 0` (timestamp vs int) | Changed to `IS NOT NULL` (common/shareddb.cpp) |
| 7 | Backticks in world/client.cpp (4 locations) | Stripped MySQL identifier quoting |
| 8 | `REGEXP` in saylink query | Changed to `!~` PG regex (common/say_link.cpp) |
| 9 | Incremental rebuild | All binaries rebuilt successfully |

### Session 2026-06-25: Zone Entry + FindReplace Fix
| # | Error | Fix |
|---|-------|-----|
| 1 | Zone boot timer too short (5s) | Increased to 30s (world/zoneserver.cpp) |
| 2 | "Already booted" corrupts world state | Re-report current zone instead of SetZoneData(0) |
| 3 | ContentFilterCriteria MySQL syntax | CONCAT/REGEXP → \|\|/~ |
| 4 | Missing perl542.dll | Copied from vcpkg submodules |
| 5 | `FindReplace("", "-")` infinite loop | Changed to `FindReplace(" ", "-")` |
| 6 | Starting items data (148 rows) | Migration 036 |
| 7 | Color column INTEGER overflow | Migration 037: changed to BIGINT |
| 8 | command_subsettings id=0, adventure_template, start_zones | Migration 038 |
| 9 | Composite PKs dropped by migration 033 | Migration 035: restored |
