# PostgreSQL Migration Map

Single source of truth for the EQEmu MySQL→PostgreSQL table migration.
Generated 2026-06-21. **Completed 2026-06-26.**

## Status: COMPLETE — 100% Parity

| Metric | Count |
|--------|-------|
| Tables in MariaDB (akk-stack) | 231 |
| Tables in PostgreSQL (ADIF) | 243 |
| Perfect schema matches | 205 |
| Tables with extra PG columns (ADIF additions) | 26 |
| Missing tables | **0** |
| Missing columns | **0** |
| PG-only tables (not in MariaDB) | 12 |

Run `python scripts/compare-pg-to-mysql.py` to verify. Full report at
`docs/postgresql-parity-report.txt`.

## Migration Timeline

| Date | Phase | What |
|------|-------|------|
| 2026-06-21 | Setup | PostgreSQL container, initial 29 migrations, first login |
| 2026-06-22 | C++ conversion | 290+ files: REPLACE INTO, FROM_UNIXTIME, backticks, Perl generator |
| 2026-06-23 | Schema alignment | Migrations 030-034: table renames, column fixes, missing tables |
| 2026-06-24 | ON CONFLICT fixes | Fixed composite PKs in C++ repos, MySQL syntax cleanup |
| 2026-06-25 | Zone entry | Migrations 035-042, boot timer, FindReplace fix, first zone entry |
| 2026-06-26 | **100% parity** | Migrations 043-044: 48 tables + all column fixes, debug cleanup |

## Migration Files

| Migration | Description |
|-----------|-------------|
| 001-008 | Core ADIF schema (accounts, zones, characters, NPCs, items, spells, factions, seed data) |
| 009-022 | Extended schema (server infra, extensions, logging, views, seed defaults) |
| 023-029 | Full EQEmu-compat schemas (doors, items, spells, zones, NPCs, misc) |
| 030 | Phase 1: 9 table renames (zones→zone, npc_templates→npc_types, etc.) |
| 031 | Phase 2: Column fixes on 8 tables (character_data rebuild to 106 cols) |
| 032 | Phase 3: 17 missing Tier 1 tables (AA system, base_data, factions) |
| 033 | Zone entry fixes (spawn2_disabled, global_loot, level_exp_mods) |
| 034 | Remaining Tier 2 tables (raids, buyer, guilds, DZ, tasks, adventures) |
| 035 | Restored composite PKs dropped by 033 |
| 036 | Starting items seed data (148 rows) |
| 037 | Color column overflow fix (INTEGER→BIGINT) |
| 038 | command_subsettings, adventure_template, start_zones fixes |
| 039 | 22 missing character/gameplay tables |
| 040 | Column fixes and table rebuilds (corpses, buffs, buyer, etc.) |
| 041 | Remaining column fixes (zone_flags casing, raid_members bot_id) |
| 042 | Final column renames (base_data end→endurance) |
| **043** | **48 missing tables for 100% parity (all categories)** |
| **044** | **All column/type/PK fixes for 100% parity** |

## C++ Code Changes (in reference/eqemu-server/)

| Category | Files | Count |
|----------|-------|-------|
| Base repository MySQL→PG | `common/repositories/base/*.h` | 250 files |
| Custom repository MySQL→PG | `common/repositories/*.h` | 27 files |
| Zone server MySQL→PG | `zone/*.cpp` | 12 files |
| Common/World/Login MySQL→PG | `common/*.cpp`, `world/*.cpp` | 8 files |
| RETURNING id for InsertOne | `common/repositories/base/*.h` | 250 files |
| Perl generator + template | `utils/scripts/generators/` | 2 files |
| RewriteQuery() gutted | `common/dbcore.cpp` | 1 file |
| Debug logging cleanup | `zone/*.cpp` | 7 files |

## PG-Only Tables (ADIF additions, not in MariaDB)

These 12 tables exist in PostgreSQL but not in MariaDB. They are legacy ADIF
tables from early migrations that predate the EQEmu compatibility work:

| Table | Notes |
|-------|-------|
| `aa_actions` | Legacy ADIF AA system |
| `aa_effects` | Legacy ADIF AA system |
| `accounts` | Legacy duplicate of `account` |
| `altadv_vars` | Legacy AA variables |
| `item_tick` | Legacy item tick data |
| `merchant_entries` | Legacy merchant data |
| `tblloginserveraccounts` | Legacy login tables |
| `tblserveradminregistration` | Legacy login tables |
| `tblserverlisttype` | Legacy login tables |
| `tblworldserverregistration` | Legacy login tables |
| `webdata_character` | Legacy web API |
| `webdata_servers` | Legacy web API |

## Intentional Schema Differences

These deliberate differences from MariaDB are documented and the C++ code
handles them correctly:

| Table | Difference | Reason |
|-------|-----------|--------|
| `base_data` | `end`→`endurance`, `end_regen`→`endurance_regen`, `end_fac`→`endurance_fac` | `end` is a PostgreSQL reserved word. C++ repository maps struct member `end` to column `"endurance"`. |
| Various tables | Extra columns in PG (email, created_at, etc.) | ADIF additions for modernization. Don't affect EQEmu C++ code. |
| Various tables | PG adds PKs where MySQL has none | PostgreSQL best practice. Doesn't affect C++ queries. |

## Verification Checkpoints

- [x] Login to server list
- [x] Select server, reach character select
- [x] Create character (name approved, saved to DB)
- [x] Enter zone (zone boots, client loads)
- [x] Move around, see NPCs (121 NPCs in Grobb)
- [ ] Combat works
- [ ] Zone transition works
- [ ] Character saves on logout
- [ ] Re-login loads saved character
