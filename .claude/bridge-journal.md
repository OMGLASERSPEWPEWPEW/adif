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
