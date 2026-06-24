# PostgreSQL Migration — Error Inventory

Complete catalog of every PostgreSQL error observed in the EQEmu server logs
after MySQL→PostgreSQL conversion. Updated 2026-06-23.

---

## 1. BLOCKERS — Fixed This Session

These prevented character creation or zone entry and have been resolved.

| # | Error | Root Cause | Fix Applied |
|---|-------|-----------|-------------|
| 1 | `EXTRACT(EPOCH FROM deleted_at)::int` — "function extract(unknown, integer)" | `deleted_at` was briefly changed to INTEGER; C++ SelectColumns assumes TIMESTAMP | Reverted deleted_at to TIMESTAMP |
| 2 | `TO_TIMESTAMP(null)` in INSERT — "column deleted_at is of type integer" | Same as above | Same fix |
| 3 | `duplicate key value violates unique constraint (id)=(0)` | MySQL AUTO_INCREMENT treats id=0 as "next ID"; PG SERIAL inserts literal 0 | Added `auto_id_on_zero()` trigger on character_data |
| 4 | InsertOne returns id=0 → `ReserveName` returns false → name rejected | InsertOne SQL had no `RETURNING id` clause; PG has no global `last_insert_id()` | Added `RETURNING id` to all 250 base repository InsertOne methods + clean rebuild |
| 5 | `RestTimer` column name case mismatch | C++ generates unquoted `RestTimer` (PG folds to `resttimer`), but DDL had quoted `"RestTimer"` | `ALTER TABLE RENAME COLUMN` to `resttimer` |
| 6 | `character_bind` missing `instance_id` column | ADIF simplified schema didn't include it; C++ INSERT/SELECT expects it | `ALTER TABLE ADD COLUMN` |
| 7 | `character_skills` — column `id` does not exist | ADIF used `character_id`; C++ expects `id` | `RENAME COLUMN character_id TO id` |
| 8 | Same for `character_spells`, `character_memmed_spells`, `character_buffs` | Same pattern | Same fix on all 4 tables |
| 9 | `group_id` — column `character_id` does not exist | C++ expects `charid` | `RENAME COLUMN character_id TO charid` |
| 10 | `starting_items` missing `class_list`, `race_list`, `deity_list`, `zone_id_list` + augment cols | Modern EQEmu schema added these columns; ADIF schema didn't have them | `ALTER TABLE ADD COLUMN` (12 columns) |
| 11 | Zone server exits immediately after LoadZones | Missing `plugins/` and `lua_modules/` dirs; `CheckForCompatibleQuestPlugins` needs files with "CheckHandin" | Created dirs + minimal plugin stubs |
| 12 | "No zoneserver available to boot up" | No zone.exe process was running | Started zone.exe |
| 13 | Tutorial zone (tutorialb/189) as start zone | `World:EnableTutorialButton` was true; `start_zones` data pointed to tutorial | Set rule to false, updated `TutorialZoneID` to -1 |

---

## 1b. BLOCKERS — Still Open (Prevent Zone Entry)

These errors fire during character creation and zone boot, preventing clean entry.

### ON CONFLICT Wrong Column (composite PK issue)
The C++ conversion changed `REPLACE INTO` to `ON CONFLICT (id)` but these tables
have composite primary keys. The upsert targets only the first PK column.

| Table | C++ ON CONFLICT | Actual PK | Fix |
|-------|----------------|-----------|-----|
| `character_bind` | `ON CONFLICT (id)` | `(id, slot)` | Change to `ON CONFLICT (id, slot)` |
| `character_skills` | `ON CONFLICT (id)` | `(id, skill_id)` | Change to `ON CONFLICT (id, skill_id)` |
| `character_languages` | `ON CONFLICT (id)` | `(id, lang_id)` | Change to `ON CONFLICT (id, lang_id)` |
| `rule_values` (~60x at startup) | `ON CONFLICT (ruleset_id)` | `(ruleset_id, rule_name)` | Change to `ON CONFLICT (ruleset_id, rule_name)` |
| `command_subsettings` | `ON CONFLICT (id)` all id=0 | `(id)` | Needs auto_id trigger + fix conflict |

### Column Name Mismatches
| Table | C++ Column | PG Column | Fix |
|-------|-----------|-----------|-----|
| `starting_items` | `item_id` | `itemid` | Rename `itemid` → `item_id` |
| `group_id` | `charid` | `character_id` | Rename `character_id` → `charid` (fix didn't persist?) |
| `level_exp_mods` | `aa_exp_mod` | missing | Add column |
| `respawn_times` | `expire_at` | missing | Add column |
| `spawn_condition_values` | `instance_id` | missing | Add column |

### Missing Tables (Zone Boot)
| Table | System | When |
|-------|--------|------|
| `spawn2_disabled` | Dynamic spawn disable | Zone loading spawns |
| `global_loot` | Global loot tables | Zone boot |
| `ldon_trap_templates` | LDoN traps | Zone boot |
| `ldon_trap_entries` | LDoN traps | Zone boot |

---

## 2. MISSING TABLES — Non-Fatal (~30 tables)

Server continues running but logs errors when querying these tables.

### Tier 2: Gameplay Features

| Missing Table | System | When Referenced |
|---------------|--------|-----------------|
| `raid_leaders` | Raid system | World startup (clearing raids) |
| `inventory_snapshots` | Inventory tracking | World startup (clearing snapshots) |
| `buyer` | Bazaar buyer/barter | World startup (clearing trader) |
| `buyer_buy_lines` | Bazaar buyer/barter | World startup |
| `buyer_trade_items` | Bazaar buyer/barter | World startup |
| `guild_permissions` | Guild permission system | World + Zone startup |
| `guild_tributes` | Guild tribute system | World + Zone startup |
| `tributes` | Tribute system | World + Zone startup |
| `tribute_levels` | Tribute level data | World startup |
| `adventure_template` | LDoN adventures | World startup |
| `adventure_template_entry` | LDoN adventure entries | World startup |
| `dynamic_zones` | Dynamic zone instances | World startup (purge + load) |
| `dynamic_zone_members` | DZ membership | World startup |
| `dynamic_zone_lockouts` | DZ lockouts | World startup |
| `dynamic_zone_templates` | DZ templates | World startup |
| `character_expedition_lockouts` | Expedition lockouts | World periodic cleanup |
| `character_task_timers` | Task cooldowns | World periodic cleanup |
| `tasks` | Task definitions | World + Zone startup |
| `task_activities` | Task steps | World + Zone startup |
| `tasksets` | Task groups | Zone startup |
| `shared_tasks` | Shared tasks | World startup |
| `shared_task_members` | Shared task members | World startup |
| `shared_task_activity_state` | Shared task progress | World startup |
| `shared_task_dynamic_zones` | Shared task DZs | World startup |
| `zone_state_spawns` | Zone state persistence | World startup (cleanup) |
| `profanity_list` | Chat profanity filter | Zone startup |
| `npc_scale_global_base` | NPC auto-scaling | Zone startup |
| `items_evolving_details` | Evolving items | Zone startup |

---

## 3. MYSQL SYNTAX IN C++ — Requires Code Fix + Rebuild

Hardcoded MySQL SQL that wasn't caught by the repository conversion script.

| # | Error | Query/Location | PG Fix |
|---|-------|---------------|--------|
| 1 | `SHOW COLUMNS FROM db_version LIKE 'custom_version'` | database_update_manifest.h | MySQL DDL — can skip (legacy schema check) |
| 2 | `INT(11) UNSIGNED NOT NULL` | database_update_manifest.h | MySQL DDL — can skip |
| 3 | `ALTER TABLE ... ADD ... AFTER column` | database_update_manifest.h | MySQL DDL — can skip |
| 4 | `UPDATE eqtime SET ... LIMIT 1` | eqtime save (periodic) | Remove `LIMIT 1` — eqtime has 1 row anyway |
| 5 | `time_of_death != 0` (timestamp vs integer comparison) | character_corpses cleanup | Change to `time_of_death IS NOT NULL` |
| 6 | Backticks in `world/client.cpp:683` | Random name generation: `` SELECT `name` FROM `character_data` `` | Strip backticks |
| 7 | Backticks in `world/client.cpp:858` | Character list query | Strip backticks |
| 8 | `REGEXP` in saylink query | `WHERE phrase not REGEXP ['A-Z']` | Change to PG regex: `WHERE phrase !~ '[A-Z]'` |

---

## 4. ON CONFLICT MISMATCHES — Requires Code Fix or DB Fix

The MySQL→PG conversion changed `REPLACE INTO` to `ON CONFLICT (column)` but
used the wrong conflict target column(s).

| # | Table | Generated ON CONFLICT | Actual PK/Unique | Fix |
|---|-------|----------------------|------------------|-----|
| 1 | `rule_values` (~60 upserts at startup) | `ON CONFLICT (ruleset_id)` | PK `(ruleset_id, rule_name)` | Change C++ to `ON CONFLICT (ruleset_id, rule_name)`, OR add unique index on `ruleset_id` alone |
| 2 | `character_languages` | `ON CONFLICT (id)` | PK `(id, lang_id)` | Change C++ to `ON CONFLICT (id, lang_id)` |
| 3 | `character_skills` | `ON CONFLICT (id)` | PK `(id, skill_id)` | Change C++ to `ON CONFLICT (id, skill_id)` |
| 4 | `character_bind` | `ON CONFLICT (id)` | PK `(id, slot)` | Change C++ to `ON CONFLICT (id, slot)` |
| 5 | `command_subsettings` | `ON CONFLICT (id)` with id=0 for all rows | PK `(id)` but all ids are 0 | Needs `auto_id_on_zero()` trigger + fix ON CONFLICT |

---

## 5. COLUMN NAME / TYPE MISMATCHES — Remaining

| # | Table | Column Issue | Status |
|---|-------|-------------|--------|
| 1 | `character_corpses` | `time_of_death` is TIMESTAMP but C++ compares with `!= 0` (integer) | C++ fix needed |
| 2 | `command_subsettings` | id=0 bulk insert fails (same AUTO_INCREMENT issue as character_data) | Needs `auto_id_on_zero()` trigger |

---

## 6. FIXED THIS SESSION — Summary

### SQL Migrations Applied
| Migration | What |
|-----------|------|
| 030_phase1_table_renames.sql | 9 table renames, FK constraint drops |
| 031_phase2_schema_fixes.sql | character_data column fixes, inventory/currency/doors/guilds column adds, satellite table column renames |
| 032_phase3_missing_tier1.sql | AA system, base_data, faction tables, data_buckets, instance_list |

### Ad-hoc SQL Fixes
| Fix | What |
|-----|------|
| `auto_id_on_zero()` trigger | Mimics MySQL AUTO_INCREMENT id=0 behavior on character_data |
| `adventure_stats` table created | Stops error spam on login |
| `deleted_at` reverted to TIMESTAMP | C++ uses EXTRACT/TO_TIMESTAMP on this column |
| `character_bind` + `instance_id` | Column required by C++ bind system |
| `character_skills/spells/memmed_spells/buffs` | Renamed `character_id` → `id` to match C++ |
| `group_id` | Renamed `character_id` → `charid` |
| `starting_items` +12 columns | class_list, race_list, deity_list, zone_id_list, augments, status, inventory_slot |
| Tutorial disabled | Rules `EnableTutorialButton=false`, `TutorialZoneID=-1` |
| Quest plugin stubs | Created `lua_modules/check_handin.lua` + `plugins/check_handin.pl` |

### C++ Changes + Rebuild
| Fix | What |
|-----|------|
| `RETURNING id` on InsertOne | Added to all 250 base repository .h files via sed |
| Clean rebuild | Full `--clean-first` cmake rebuild of all server binaries |
