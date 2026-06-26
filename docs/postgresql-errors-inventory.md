# PostgreSQL Migration — Error Inventory

Complete catalog of every PostgreSQL error encountered and fixed during the
EQEmu MySQL→PostgreSQL conversion. Updated 2026-06-26.

---

## STATUS: ALL ERRORS RESOLVED

As of 2026-06-26, the PostgreSQL database has **100% schema parity** with the
akk-stack MariaDB ground truth (231/231 tables). The comparison script
`scripts/compare-pg-to-mysql.py` confirms zero missing tables and zero missing
columns. See `docs/postgresql-parity-report.txt` for the full report.

### Remaining Cosmetic Issues (Non-blocking)

MySQL DDL introspection strings in `database_update_manifest.h` produce
harmless log noise on world startup. These are legacy MySQL commands that the
server tries to run but that don't affect runtime behavior:

| Error | Query |
|-------|-------|
| `SHOW COLUMNS FROM db_version LIKE 'custom_version'` | MySQL introspection |
| `INT(11) UNSIGNED NOT NULL` | MySQL type syntax in DDL |
| `ALTER TABLE ... ADD ... AFTER column` | MySQL positional ALTER |
| `show columns from db_version where Field like '%bots_version%'` | MySQL introspection |

---

## FIXED — Complete History

### Session 2026-06-21: Initial PostgreSQL Setup
| # | Error | Fix |
|---|-------|-----|
| 1 | MariaDB crash-loop (InnoDB redo log version) | Upgraded Dockerfile to mariadb:10.11 |
| 2 | login.json port type (string vs int) | Changed to integer 5433 |
| 3 | Account id=0 treated as "not found" | Updated to id=1, reset sequence |

### Session 2026-06-22: C++ MySQL→PostgreSQL Conversion (290+ files)
| # | Error | Fix |
|---|-------|-----|
| 1 | `REPLACE INTO` across 244 base repos | Converted to `INSERT...ON CONFLICT DO UPDATE` |
| 2 | `FROM_UNIXTIME()` (270 instances) | Converted to `TO_TIMESTAMP()` |
| 3 | `UNIX_TIMESTAMP()` (54 instances) | Converted to `EXTRACT(EPOCH FROM)::int` |
| 4 | Backtick identifier quoting (900+ instances) | Stripped across all files |
| 5 | `ON DUPLICATE KEY UPDATE` (10 custom repos) | Converted to `ON CONFLICT DO UPDATE/NOTHING` |
| 6 | `IFNULL()` → `COALESCE()` | login_accounts, database_instances |
| 7 | `FIELD()` → `array_position()/CASE` | expedition_lockouts |
| 8 | `GROUP_CONCAT()` in manifests | Skipped (legacy MySQL DDL, not runtime) |
| 9 | Perl generator outputs MySQL SQL | Fixed template to output PG-native SQL |
| 10 | `RewriteQuery()` runtime translation | Gutted to passthrough |
| 11 | Backtick in C++ char literals (database.cpp, strings_legacy.cpp) | Fixed compile errors |

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
| 14 | 5 column name/missing column issues | Migration 033: zone entry fixes |
| 15 | ~26 missing Tier 2 tables | Migration 034: raids, buyer, guilds, tributes, DZ, tasks, etc. |
| 16 | 4 missing zone boot tables | Migration 033: spawn2_disabled, global_loot, ldon_trap_* |

### Session 2026-06-24: ON CONFLICT Fixes + MySQL Syntax Cleanup
| # | Error | Fix |
|---|-------|-----|
| 1 | `ON CONFLICT (id)` on character_bind (PK is id,slot) | Fixed PrimaryKey() → `"id, slot"` |
| 2 | `ON CONFLICT (id)` on character_skills (PK is id,skill_id) | Fixed PrimaryKey() → `"id, skill_id"` |
| 3 | `ON CONFLICT (id)` on character_languages (PK is id,lang_id) | Fixed PrimaryKey() → `"id, lang_id"` |
| 4 | `ON CONFLICT (ruleset_id)` on rule_values | Fixed PrimaryKey() → `"ruleset_id, rule_name"` |
| 5 | `UPDATE eqtime ... LIMIT 1` | Removed LIMIT (common/database.cpp) |
| 6 | `time_of_death != 0` (timestamp vs int) | Changed to `IS NOT NULL` (common/shareddb.cpp) |
| 7 | Backticks in world/client.cpp (4 locations) | Stripped MySQL identifier quoting |
| 8 | `REGEXP` in saylink query | Changed to `!~` PG regex (common/say_link.cpp) |

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
| 10 | 22 missing character/gameplay tables | Migration 039 |
| 11 | 12 column mismatches (corpses, spells, buffs, etc.) | Migration 040-042 |

### Session 2026-06-26: 100% Parity Achieved
| # | Error | Fix |
|---|-------|-----|
| 1 | 48 remaining missing tables | Migration 043: all tables created (incl. Bot, Merc, Spire, player_event) |
| 2 | Column mismatches across 30+ tables | Migration 044: type widening, missing cols, PK corrections |
| 3 | Boolean→SMALLINT default cast failures | Drop default → alter type → re-add default |
| 4 | Integer→TIMESTAMP cast failures | Drop default → alter with USING TO_TIMESTAMP() |
| 5 | `adventure_template.graveyard_radius` TEXT→REAL | Drop default → alter with USING cast |
| 6 | `petitions.senttime` TIMESTAMP→BIGINT | Alter with USING EXTRACT(EPOCH FROM) |
| 7 | `trader` table schema completely wrong (6 vs 18 cols) | DROP + CREATE with correct 19-column schema |
| 8 | `character_corpse_items` missing 11 columns | Added aug_1-6, attuned, custom_data, ornament cols |
| 9 | `group_leaders` missing 7 columns | Added marknpc, leadershipaa, maintank, assist, puller, mentoree, mentor_percent |
| 10 | `raid_details` missing 10 columns | Added motd + 9 marked_npc columns |
| 11 | 8 primary key mismatches | Corrected composite PKs across tables |
| 12 | Debug logging in C++ zone code | Removed [PG-TIMING], [HQS], [HQSL], [AddNPC], [PZ-STEP], [SZC], std::cerr |

### C++ Code Fixes (All Sessions, Applied to reference/eqemu-server/)

| # | File | Fix | Description |
|---|------|-----|-------------|
| 1 | `world/zoneserver.cpp` | `zone_boot_timer(5000)` → `(30000)` | Increased boot timer for PG |
| 2 | `zone/zone.cpp` | `SetZoneData(0)` → `SetZoneData(zone->GetZoneID(), ...)` | Fix "already booted" state corruption |
| 3 | `common/repositories/criteria/content_filter_criteria.h` | `CONCAT`/`REGEXP` → `\|\|`/`~` | PostgreSQL string/regex syntax |
| 4 | `zone/quest_parser_collection.cpp` | `FindReplace("", "-")` → `FindReplace(" ", "-")` | Fix infinite loop on empty string match |
| 5 | 250 base repository files | Added `RETURNING id` to InsertOne | PostgreSQL doesn't return last_insert_id() |
| 6 | `zone/zone.cpp` | Removed [PG-TIMING] logging + timing vars | Debug cleanup |
| 7 | `zone/quest_parser_collection.cpp` | Removed [HQS]/[HQSL] std::cerr | Debug cleanup |
| 8 | `zone/entity.cpp` | Removed [AddNPC] logging | Debug cleanup |
| 9 | `zone/spawn2.cpp` | Removed [PZ-STEP] logging | Debug cleanup |
| 10 | `zone/npc.cpp` | Removed [SZC] logging | Debug cleanup |
| 11 | `zone/zonedb.cpp` | Replaced std::cerr with LogError | Debug cleanup |
| 12 | `zone/questmgr.cpp` | Replaced std::cerr with LogError | Debug cleanup |
