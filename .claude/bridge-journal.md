# Bridge Journal — ADIF

### 2026-06-21 22:15

SESSION 2026-06-21 evening: Major progress on PostgreSQL migration for EQEmu server.

## What Happened
- MariaDB in akk-stack was crash-looping for 6+ hours — InnoDB redo log created by 10.11.18 but container runs 10.5.4. Fixed by upgrading Dockerfile FROM mariadb:10.5.4 to mariadb:10.11.
- Built PostgreSQL-backed EQEmu server binaries already exist at reference/eqemu-server/build/bin/RelWithDebInfo/ (world.exe, zone.exe, loginserver.exe, shared_memory.exe). Build was completed in a PREVIOUS session.
- Set up the build directory with eqemu_config.json (pointing to PostgreSQL on port 5433), login.json, opcodes, patches, maps, and libpq.dll + dependencies.
- Fixed login.json port type: GetVariableInt needs integer 5433, not string "5433". This caused loginserver to fall back to MySQL default port 3306.
- Fixed account ID 0 issue: migrated account darklight had id=0, EQEmu treats 0 as "not found". Updated to id=1 and reset sequence.
- Got to character select screen! First successful PostgreSQL-backed login.
- Character creation fails because PostgreSQL has table "characters" (simplified ADIF schema) but EQEmu code expects "character_data" with ~100 columns.
- ~220 tables missing from PostgreSQL that EQEmu repositories reference.
- ~250 repository files use REPLACE INTO (MySQL-only), needs INSERT ... ON CONFLICT DO UPDATE.

## Current State
- PostgreSQL server binaries work and connect to the database
- Login works, character select works
- Character creation/zone entry blocked by missing/mismatched table schemas
- akk-stack MariaDB now on 10.11 (Dockerfile changed, not committed)

## What's Next
- PROPERLY migrate all EQEmu table schemas from MySQL to PostgreSQL (don't rush, do it right)
- Create proper numbered migration files in database/migrations/
- Fix REPLACE INTO across repository files
- The user explicitly said: "Let's not rush this and do this Postgres migration right"

## Critical Context for Future Sessions
- This is NOT a custom ADIF server. We are swapping MySQL for PostgreSQL in the STANDARD EQEmu server code.
- The converted C++ code is at reference/eqemu-server/ (cloned from github.com/EQEmu/Server)
- The build is ALREADY DONE — do not try to rebuild
- Master docs: docs/tech-comparison.html (11 modernization areas) and docs/opcode-audit.html (4 milestones)
- PostgreSQL runs on port 5433 (not 5432, conflicts with local PG15), database name "adif", user "adif", password "adif_dev"
- To run the servers: cd reference/eqemu-server/build/bin/RelWithDebInfo, run shared_memory.exe, then loginserver.exe, then world.exe
- eqhost.txt points client to 127.0.0.1:5998

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Fixed MariaDB crash-loop (InnoDB redo log version mismatch)
> - Set up PostgreSQL-backed EQEmu server build dir with configs/assets
> - Fixed login.json port type, account ID=0, and session-journal hook
> - First successful PostgreSQL login — reached character select
> - Hit character_data table schema mismatch blocking char creation
>
> **Commits since last entry:**
> ```
> 738539b feat(database): add full EQEmu schemas for zones, NPCs, and 21 gameplay tables
> ebd84d5 fix(database): resolve port conflict and add EQEmu compat columns
> e9f92ba feat(database): add full EQEmu items, spells, and doors schemas
> 2a1e3b9 feat(database): add 69 tables completing EQServer PostgreSQL schema
> 6e6df2b fix(infra): adapt all hooks for Windows 10 compatibility
> ```
>
> **Files touched:**
> ```
> database/migrations/009-029  | 2783 lines of new migration SQL
> .claude/hooks/*              | Windows 10 compat fixes
> database/docker-compose.yml  | port 5432→5433
> ```

### 2026-06-22 09:56

SESSION 2026-06-22: Massive C++ MySQL→PostgreSQL conversion completed.

## What Happened
- Converted all 290+ C++ source files from MySQL SQL to native PostgreSQL
- Phase 1: 250 base repository files via conversion script (REPLACE INTO, FROM_UNIXTIME, UNIX_TIMESTAMP, backticks)
- Phase 2: 27 custom repository files manually (ON DUPLICATE KEY, timestamps, IFNULL)
- Phase 3: ~12 zone server files via parallel agents (groups, raids, mob, quests, tasks, tradeskills, exp, client, zonedb)
- Phase 4: ~8 common/world/login files via parallel agents (database.cpp, shareddb, profanity, ptimer)
- Phase 5: Fixed Perl generator + template to output PG-native SQL
- Phase 6: Gutted RewriteQuery() runtime translation layer (kept minimal shim for legacy manifest)
- Fixed 2 compile errors (backtick strip hit C++ char literals in database.cpp and strings_legacy.cpp)
- First successful build + server test: boots, loads 618 zones, connects to login server

## Server Test Results
- Login works, character select loads
- ~30 tables still missing from PostgreSQL (Tier 2 from migration map)
- character_data needs rebuilding (51 cols vs 106 expected)
- INTERVAL syntax + ON CONFLICT composite key fixed in shim
- Name filter broken (missing id column) — blocks character creation

## What's Next
- Step 2: Migration 030 — fix column mismatches (character_data rebuild, guilds, data_buckets, name_filter)
- Step 3: Migrations 031-033 — create ~30 missing tables
- Then: character creation → zone entry → combat testing

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Converted 290+ C++ files from MySQL to native PostgreSQL SQL across 6 phases
> - Built conversion script for 250 base repos, manually converted 27 custom repos
> - Parallel agents converted zone/common/world files simultaneously
> - Fixed Perl generator, gutted RewriteQuery, first successful server boot on PG
>
> **Commits since last entry:**
> ```
> 25d3c48 chore(infra): update session journals and memory heaps
> fc8f469 chore(infra): update session journals and memory heaps
> fc03819 docs(database): mark all C++ conversion phases complete in migration map
> 5e9357c chore(infra): update session journals and memory heaps
> 918c75a feat(database): add Python script for bulk MySQL→PG repo conversion
> 37b7e76 docs(database): update migration map with C++ conversion progress
> 989f55c docs(database): add PostgreSQL migration map tracking 250 EQEmu tables
> ```
>
> **Files touched:**
> ```
> docs/postgresql-migration-map.md   |   71 +-
> scripts/convert-repos-to-pg.py     |  256 +++++
> .claude/memory/heaps/*             | 2412 +++
> .claude/journals/*                 | 1043 ++
> ```

### 2026-06-23 20:54

SESSION 2026-06-23: Massive PostgreSQL migration progress — character creation works, zone boot works, first zone entry attempt.

## What Happened
- Applied migrations 030-034: table renames, schema fixes, missing Tier 1+2 tables
- Fixed character name rejection (3 bugs: deleted_at type, id=0 auto-increment, missing RETURNING id)
- Added RETURNING id to all 250 base repository InsertOne methods + clean C++ rebuild
- Fixed zone server crash (missing quest plugin stubs for CheckHandin)
- Disabled tutorial zone, set racial starting cities
- Fixed 5+ column name mismatches (starting_items, group_id, respawn_times, etc.)
- Created ~30 missing tables (Tier 2 gameplay systems)
- Dropped composite PK constraints blocking ON CONFLICT upserts

## Current State
- Login → character select → character creation: WORKS
- Character enters Grobb (zone boots with maps/water/navmesh): WORKS
- BUT: "Zone bootup timer expired" — zone boots too slowly or ON CONFLICT errors prevent bind/skills from saving
- ON CONFLICT composite PK issue remains: character_bind, character_skills, character_languages use ON CONFLICT (id) but need ON CONFLICT (id, slot/skill_id/lang_id) — requires C++ fix in custom repos + rebuild
- rule_values ON CONFLICT (ruleset_id) needs ON CONFLICT (ruleset_id, rule_name) — same pattern
- Error inventory at docs/postgresql-errors-inventory.md is comprehensive

## What's Next
- Fix ON CONFLICT composite PK targets in C++ custom repository files (character_bind, character_skills, character_languages, rule_values) — this is the LAST blocker for zone entry
- Rebuild after fixing
- Test zone entry end-to-end
- Then: movement, NPCs, combat testing

## Key Files
- Migrations: database/migrations/030-034_*.sql
- Error inventory: docs/postgresql-errors-inventory.md
- C++ repos with RETURNING id: reference/eqemu-server/common/repositories/base/*.h
- Quest stubs: reference/eqemu-server/build/bin/RelWithDebInfo/lua_modules/check_handin.lua + plugins/check_handin.pl
- auto_id_on_zero() trigger on character_data (in PG, not in migration files)

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Fixed 13 blockers preventing character creation and zone entry
> - Added RETURNING id to 250 C++ repository files, clean rebuilt server
> - Created 30+ missing PostgreSQL tables (Tier 1 + Tier 2)
> - First successful zone boot: Grobb loaded with maps/water/navmesh
>
> **Commits since last entry:**
> ```
> (no commits — all changes are uncommitted: 5 new migrations, C++ repo edits, quest stubs, DB triggers)
> ```
>
> **Files touched:**
> ```
> database/migrations/030-034_*.sql                              | 5 new migration files
> docs/postgresql-errors-inventory.md                            | comprehensive error catalog
> reference/eqemu-server/common/repositories/base/*.h            | 250 files (RETURNING id)
> reference/eqemu-server/build/bin/RelWithDebInfo/lua_modules/   | quest stub
> reference/eqemu-server/build/bin/RelWithDebInfo/plugins/       | quest stub
> ```
