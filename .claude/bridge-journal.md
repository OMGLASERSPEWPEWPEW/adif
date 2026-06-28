
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

### 2026-06-25 19:01

SESSION 2026-06-25: ZONE ENTRY WORKS — Character in Grobb on PostgreSQL.

## What Happened
- Fixed zone boot timer (5s→30s), added PG-TIMING instrumentation to Zone::Init
- Fixed "already booted" handler (SetZoneData(0) → re-report current zone)
- Fixed ContentFilterCriteria MySQL syntax (CONCAT/REGEXP → ||/~)
- Copied perl542.dll from vcpkg for embedded Perl runtime
- Fixed FindReplace("", "-") infinite loop in quest_parser_collection.cpp:1072 → FindReplace(" ", "-")
- Zone boots Grobb in ~6 seconds, 121 NPCs spawned, character enters zone

## Current State
- Login → character select → character creation → zone entry: ALL WORK on PostgreSQL
- ~25 missing tables and ~15 column mismatches producing non-fatal errors during zone boot and character load
- Error inventory needs updating with all new errors from zone boot log
- Debug logging (std::cerr, PG-TIMING, [PZ-STEP], [SZC], [AddNPC], [HQS], [HQSL]) still in C++ code — needs cleanup

## What's Next
- Document all remaining PostgreSQL errors into error inventory
- Create migrations for missing tables + column fixes
- Clean up debug logging from C++ code
- Test combat, NPC interaction, zone transitions

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Debugged zone boot hang through 8+ iterations of narrowing (timer→spawns→quest parser→FindReplace infinite loop)
> - Found root cause: `Strings::FindReplace(npc_name, "", "-")` — empty string match causes infinite loop
> - Character "Ghouldan" entered Grobb with 121 NPCs spawned, zone running on PostgreSQL
> - Cataloged ~25 missing tables and ~15 column mismatches from zone boot + character load errors
>
> **Commits since last entry:**
> ```
> c130bb1 chore(infra): update session journals, memory heaps, and bridge journal
> 60b6d63 fix(infra): improve conversation logger with tool collapsing and system-reminder stripping
> 02cea27 docs(database): add PostgreSQL migration error inventory
> 18f51b9 feat(database): add migrations 030-034 for EQEmu PostgreSQL schema alignment
> ```
>
> **Files touched (uncommitted C++ changes):**
> ```
> reference/eqemu-server/zone/zone.cpp                    | PG-TIMING, boot timer 30s, cerr output
> reference/eqemu-server/zone/spawn2.cpp                  | [PZ-STEP] logging
> reference/eqemu-server/zone/npc.cpp                     | [SZC] logging
> reference/eqemu-server/zone/entity.cpp                  | [AddNPC] logging + cerr
> reference/eqemu-server/zone/quest_parser_collection.cpp | FindReplace fix + [HQS]/[HQSL] logging
> reference/eqemu-server/world/zoneserver.cpp              | boot timer 5s→30s
> reference/eqemu-server/common/repositories/criteria/content_filter_criteria.h | CONCAT/REGEXP→PG
> ```

### 2026-06-25 22:28

SESSION 2026-06-25 (late): Migrations 039-042 applied, most PG errors resolved. Character in Grobb with 121 NPCs. Zone transitions attempted (Grobb→Innothule) — needs 2+ zone processes and a few remaining column fixes.

## Next Session TODO
1. **Two interactive HTML docs** (using /html skill + Vite localhost):
   - **ADIF Roadmap doc**: 11 improvement areas, 4 milestones, player benefit analysis, interactive charts showing EQ vs ADIF capabilities, Rust server decision rationale
   - **PostgreSQL Migration doc**: What we've done so far — 42 migrations, 290+ C++ files converted, timeline of fixes, error inventory visualization
2. **Remaining PG errors**: zone_flags column casing (charID→charid in C++), base_data endurance rename test, inventory ON CONFLICT test, a few straggler columns
3. **Debug logging cleanup**: Remove all std::cerr, [HQS], [HQSL], [AddNPC], [PZ-STEP], [SZC], [PG-TIMING] instrumentation from C++ code
4. **Zone transitions**: Test with 2+ zone processes after remaining fixes
5. **Server language decision**: Leaning Rust (bevy_ecs, fearless concurrency, memory safety). Need to document rationale.

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Created 22 missing PG tables, fixed 15+ column mismatches (migrations 039-042)
> - Cross-referenced all schemas against akk-stack MariaDB ground truth
> - Rebuilt character_buffs, buyer, account_flags tables from scratch
> - Fixed base_data reserved word (end→endurance) and inventory composite PK in C++
>
> **Commits since last entry:**
> ```
> 57af650 chore(infra): update session journals and memory heaps
> b58cdad feat(database): add migration 042 for final column renames and fixes
> 1eba3aa feat(database): add migrations 039-041 for missing tables, column fixes, and remaining cleanup
> 4096a50 docs(database): update error inventory with zone entry errors and session 2026-06-25 fixes
> a89f328 feat(database): add migrations 035-038 for composite PKs, starting items, color overflow, and remaining fixes
> ```
>
> **Files touched:**
> ```
> database/migrations/039-042_*.sql                  | 490 lines of new migration SQL
> docs/postgresql-errors-inventory.md                | full rewrite with remaining errors
> reference/eqemu-server/common/repositories/base/   | base_data, inventory PK fixes
> ```

### 2026-06-26 10:01

SESSION 2026-06-26: 100% PostgreSQL PARITY ACHIEVED.

## What Happened
- Built `scripts/compare-pg-to-mysql.py` — connects to both akk-stack MariaDB (peq) and ADIF PostgreSQL (adif), compares every table and column. This is the ground truth document.
- Initial comparison: 119/231 (51.5%) — 48 missing tables, 73 with column issues
- Created migration 043: all 48 missing tables (Character, Content, Server, State, Login, Version, Bot, Merc, Spire, player_event — everything)
- Created migration 044: fixed all column mismatches (buyer_buy_lines, character_corpse_items, group_leaders, raid_details, trader rebuild, 20+ type fixes, 8 PK corrections)
- Fixed boolean→smallint default cast issues (drop default, convert, re-add)
- Fixed integer→timestamp cast issues (completed_shared_tasks, character_stats_record)
- Cleaned all debug logging from C++ zone code: [PG-TIMING], [HQS], [HQSL], [AddNPC], [PZ-STEP], [SZC], std::cerr
- Rebuilt server — all binaries compile clean (zone.exe, world.exe, loginserver.exe, shared_memory.exe)
- Final parity: 231/231 (100.0%) — 205 perfect matches, 26 with extra PG columns (ADIF additions, not issues)

## Current State
- PostgreSQL migration is COMPLETE
- Comparison script is rerunnable for verification
- Server is rebuilt with clean debug-free code
- Parity report saved to docs/postgresql-parity-report.txt

## What's Next
- End-to-end testing: login → zone entry → zone transitions → character save/reload
- Update error inventory and migration map docs
- Then: the /html artifact docs (ADIF Roadmap + PostgreSQL Migration report)
- Then: zone transition testing with 2+ zone processes

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Built schema comparison script (MariaDB vs PostgreSQL, 231 tables)
> - Created migrations 043-044: 48 missing tables + all column/type/PK fixes
> - Cleaned all debug logging from 7 C++ zone server files
> - Rebuilt server binaries — clean compile, zero warnings
>
> **Commits since last entry:**
> ```
> (no commits yet — all changes uncommitted: 2 migrations, comparison script, C++ cleanup, parity report)
> ```
>
> **Files touched:**
> ```
> scripts/compare-pg-to-mysql.py                     | 450+ new (comparison tool)
> database/migrations/043_missing_tables_full_parity.sql | 885 lines
> database/migrations/044_column_fixes_full_parity.sql   | 490 lines
> docs/postgresql-parity-report.txt                  | ground truth output
> reference/eqemu-server/zone/*.cpp                  | 7 files (debug cleanup)
> ```

### 2026-06-27 15:30

SESSION 2026-06-27: Milestone 2 COMPLETE, Milestone 3 PLANNED AND APPROVED.

## Milestone 2 Complete (all 6 phases in one session)
- buf 1.50.0 installed, lint/build/breaking all clean
- struct-proto-map.html audit doc (4 tabs, field-level EQ↔proto mapping)
- ipc.proto: 77 IPC messages replacing 226 ServerOP_ constants
- Expanded Spawn (27→43), ItemDefinition (12→40), PlayerProfile (39→57)
- proto-check.ps1 CI script, proto/README.md
- Rust codegen via prost-build, 20 Rust tests passing (117 total: 97 C# + 20 Rust)
- protoc 28.3 installed, Rust toolchain updated to 1.96.0

## Milestone 3: Rust Zone Server — PLANNED AND APPROVED
- 12 phases, ~27-35 sessions estimated
- Stack confirmed: Rust, tokio, bevy_ecs (standalone), sqlx, prost, tracing
- Workspace at server/ with adif-proto, adif-common, adif-zone crates
- Full plan at .claude/plans/ok-can-you-spin-async-clover.md

## Other Session Work
- Fixed looting (loottable had 0 rows — migrated 26,514 from MariaDB + base_data, AA tables, books, etc.)
- Ghouldan set to GM status 250
- Server launch process documented in CLAUDE.md (port 5906 for docs)
- Created scripts/docs-server.py with navigation logging

## What's Next
- Update stack.md and CLAUDE.md to commit to Rust
- Phase 1: Cargo workspace, config, sqlx DB pool, zone config loading
- Create docs/zone-server-status.html tracker

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Completed all 6 phases of Milestone 2 (Protobuf Protocol Layer) in one session
> - Fixed loot system (migrated 26,514 loottable rows + base_data, AA, books from MariaDB)
> - Planned and approved Milestone 3 (Rust Zone Server) — 12 phases, full roadmap
> - Confirmed Rust as official server language (replacing C#/.NET in stack.md)
>
> **Commits since last entry:**
> ```
> 54d7d92 feat(proto): add Rust codegen with prost-build and 20 round-trip tests (Phase 6)
> b7175dc docs(proto): add CI script, proto README, update CLAUDE.md (Phase 5)
> f817fcd feat(proto): expand Spawn, ItemDefinition, PlayerProfile to full coverage (Phase 4)
> ec0dfac chore(infra): update session journals and memory heaps
> fd886c9 docs(proto): add struct-proto audit, docs server, update CLAUDE.md
> edd1de7 feat(proto): add IPC protocol, fix lint, verify toolchain (Phases 1-3)
> 0eae0ba chore(infra): update session journals, memory heaps, and bridge journal
> 880cf56 docs(architecture): add interactive HTML artifacts for EQ architecture study
> ```
>
> **Files touched:**
> ```
> proto/adif/ipc.proto             | 692 +++ (new)
> proto/adif/inventory.proto       | 104 ++
> proto/adif/entity.proto          |  41 +-
> proto/adif/character.proto       |  47 +-
> docs/struct-proto-map.html       | 521 +++ (new)
> tests/proto-rust/src/main.rs     | 175 +++ (new)
> tests/ProtoRoundTrip/Program.cs  | 337 ++
> scripts/proto-check.ps1          |  83 +++ (new)
> scripts/docs-server.py           |  31 +++ (new)
> proto/README.md                  |  59 +++ (new)
> ```

### 2026-06-27 23:57

Session 2026-06-27: MASSIVE session. Completed ALL 12 phases of Milestone 3 (Rust Zone Server) and started the EQ protocol bridge.

## Milestone 3 Complete
- All 12 phases done in one session: Foundation, ECS, Spawn System, Game Loop, TCP Networking, Movement, NPC AI, Combat, Chat, Zone Transitions, Geometry Traits, Integration Test
- 60 tests, 119 NPCs in Grobb, 31 Hz game loop, TCP on port 7000
- server/ workspace with adif-proto, adif-common, adif-zone crates

## Protocol Bridge Progress
- Built adif-bridge crate: EQ UDP reliability protocol (session, CRC, compression, fragmentation)
- BREAKTHROUGH: DES-encrypted login response (zero-key CBC) — EQ Titanium client shows "ADIF Dev" in server list!
- Login flow working: handshake → credentials → encrypted LoginAccepted → server list → play request approved
- Next blocker: World server phase (OP_SendLoginInfo → character list → zone redirect)

## What's Next
1. Implement world server handler: proactive GuildsList+ApproveWorld packets, handle OP_SendLoginInfo, send character list, handle OP_EnterWorld, send OP_ZoneServerInfo pointing to ourselves
2. Then zone entry sequence (already built): OP_ZoneEntry → PlayerProfile + Spawns + NewZone
3. Key reference: world/client.cpp lines 441-1070 for world packet handlers

## Key Technical Discoveries
- EQ login protocol uses DES CBC encryption with all-zero key/IV for LoginAccepted
- Server list uses null-terminated strings (not length-prefixed)
- LoginBaseMessage (10 bytes) is the header on all login packets
- Session response crc_bytes=0 and encode_passes=None works for disabling encoding
- Connection phase detection by tracking connection count per IP (1st=login, 2nd=world, 3rd=zone)

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Built entire Rust zone server in 12 phases (ECS, spawns, game loop, networking, AI, combat, chat)
> - Built EQ protocol bridge with UDP reliability layer and DES encryption
> - EQ Titanium client successfully shows "ADIF Dev" in server select screen
> - Debugged login protocol through 5 iterations (CRC, compression, offsets, strings, DES)
>
> **Commits since last entry:**
> ```
> a8e2274 chore(infra): update session journal
> f6f3e2a feat(server): EQ client sees "ADIF Dev" in server list via protocol bridge
> 0528018 feat(server): add EQ-to-protobuf protocol bridge with zone entry state machine
> 667528e docs(server): mark Milestone 3 complete — 12/12 phases, 100% progress
> cf93ee1 feat(server): complete Milestone 3 — AI, combat, chat, zones, geometry (Phases 7-12)
> 97a3872 feat(server): add TCP networking, movement validation (Phases 5-6)
> 0d6039c feat(server): add spawn system and game loop (Phases 3-4)
> 1f3be96 feat(server): add Rust zone server workspace with ECS (Phases 1-2)
> ... and 8 more (docs, infra)
> ```
>
> **Files touched:**
> ```
> server/adif-zone/src/**          | 2800+ lines (zone server)
> server/adif-bridge/src/**        | 1900+ lines (protocol bridge)
> docs/zone-server-status.html     | 128 ++
> docs/rust-server.html            | 600+ (new)
> scripts/StartBridge.bat          | 24 (new)
> ```

### 2026-06-28 02:27

SESSION 2026-06-28: World Server crate built, multi-port bridge, login→world transition SOLVED.

## What Happened
- Created adif-world library crate (5 modules: account, character, zone_registry, zone_routing, lib)
- DB-backed world handler: queries real accounts (by ID) and characters from character_data table
- BREAKTHROUGH: Found EQ client uses THREE separate UDP ports (5998 login, 9000 world, 7000-7999 zone)
- Rewrote bridge main.rs with tokio::select! across 3 sockets — phase determined by port, not connection count
- Fixed play response sequence echo (was hardcoded 4, must echo client's sequence 5)
- Fixed account lookup: OP_SendLoginInfo sends lsid (integer), not account name
- Built proper Titanium CharacterSelect_Struct (1704 bytes, parallel arrays of 10 slots)
- Added packet fragmentation support for oversized UDP datagrams
- Updated 6 HTML docs to reflect M3 complete + bridge + world server progress

## Current State
- Login → server list → play approval → world connection: ALL WORKING
- Account lookup from DB: WORKING (account_id=1, status=250)
- Character list from DB: WORKING (characters=1, Ghouldan found)
- Client receives character list AND sends back CRC/checksum data
- Character select screen renders BLACK — client IS processing our data but model isn't rendering
- Likely cause: CharacterSelect_Struct appearance/equipment data fields need correct values

## What's Next
1. Fix character select rendering (try alt+enter for fullscreen toggle, check if UI is there but black)
2. Compare CharacterSelect_Struct byte-for-byte with EQEmu output (may need equipment/color data)
3. Handle OP_EnterWorld (character selection) → ZoneServerInfo → zone entry
4. Then: zone phase with real character data from DB

## Key Technical Discoveries
- EQ Titanium connects to 3 ports: 5998 (login), 9000 (world), 7000+ (zone) — NOT all on one port
- Play response MUST echo client's sequence number from PlayRequest (field 0-3 of LoginBaseMessage)
- OP_SendLoginInfo sends the login server account ID (lsid) as the identifier, not the account name
- Titanium CharacterSelect_Struct is 1704 bytes with parallel arrays, not per-character entries
- Unknown820 and Unknown902 fields must be 0xFF, unused names must be "<none>"
- Client sends 32KB CRC data (OP_CrashDump 0x7825) after receiving character list — can be ignored

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Built adif-world crate with account lookup, character queries, zone registry
> - Solved multi-port mystery: EQ client uses 5998/9000/7778 for login/world/zone
> - Fixed play response sequence echo and account ID lookup from lsid
> - Client reaches character select (sends CRC data back) but renders black
>
> **Commits since last entry:**
> ```
> 5be2d64 docs(server): update HTML docs with M3 complete, bridge progress, world server
> 4d9de10 feat(server): multi-port bridge with DB-backed world phase
> 1739927 feat(server): add adif-world crate with DB-backed world server logic
> ```
>
> **Files touched:**
> ```
> server/adif-world/src/**                 | 271 +++ (new crate, 5 modules)
> server/adif-bridge/src/main.rs           | 385 ++++++++++++-----------
> server/adif-bridge/src/world_handler.rs  | 213 ++++++++------
> docs/*.html                              | 425 ++++++++++++++++++-----
> ```

### 2026-06-28 17:25

CRC FIX ATTEMPT FAILED - Session blocked, need proper investigation. The crc32fast Hasher approach (key_le_bytes prepended to data) produces wrong values. Need to verify against capture data with known key/data/CRC triplets before next attempt. Reverted to crc_bytes=0 so bridge works while we investigate. This has been hours of circling — next session needs to start with capture verification, not guessing.

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Built UDP packet proxy, captured full EQEmu login+world flow
> - Fixed world packet sequence (removed premature ApproveWorld, correct ordering)
> - Fixed Combined handler to process Fragment sub-packets (stopped retransmit loop)
> - Added SessionDisconnect after ZoneServerInfo, DB-backed PlayerProfile
> - CRC algorithm ported from EQEmu C++ but still produces wrong values
> - Zone transition blocked: client never connects to zone port after ZoneServerInfo
>
> **Commits since last entry:**
> ```
> 691b800 docs(server): add EQ protocol reference and UDP packet capture proxy
> 7f69319 feat(server): fix world phase packet sequence and DB-backed zone entry
> a9d2d34 chore: add log and stackdump files to .gitignore
> ```
>
> **Files touched:**
> ```
> docs/eq-world-protocol.html                | 1496 +++ (new, 5-tab protocol reference)
> scripts/udp-proxy.py                       |  315 +++ (new, multi-port capture proxy)
> server/adif-bridge/src/main.rs             |  115 +- (Combined handler, CRC, zone entry)
> server/adif-bridge/src/world_handler.rs    |   94 +- (packet sequence, MOTD, disconnect)
> server/adif-bridge/src/titanium/structs.rs |  124 +- (full PlayerProfile, CharSelect)
> server/adif-world/src/character.rs         |   29 +- (appearance fields, load_by_name)
> ```

### 2026-06-28 20:40

SESSION 2026-06-28 evening: Major protocol debugging session. Three critical fixes landed + zone capture achieved.

## What Was Fixed
1. OP_WorldComplete handler — client now sends WorldComplete before disconnect, zone connection established
2. Outgoing packet fragmentation — SendCharInfo (1704 bytes) properly split into 4 fragments within max_packet_size=512
3. PlayerProfile CRC32 checksum — EQ checksum computed over bytes 4..end, raw accumulator (no NOT)
4. Bridge encode_key changed from random to 0 (matching EQEmu)
5. UDP proxy zone port rewriting — rewrites ZoneServerInfo port 7001→7000 with CRC recalculation
6. /runbridge skill created for launching bridge in visible CMD window

## Current Blocker
Client connects to zone, receives PlayerProfile (39 fragments) + player spawn, acks everything, then immediately disconnects. Client never sends OP_ReqNewZone. Loading bar shows "loading character profiles" then hangs.

## Key Discovery from Zone Capture
EQEmu zone uses encode_pass1=1 (zlib compression). Every zone packet has 0x5A (compressed) or 0xA5 (uncompressed) prefix byte. Our bridge uses encode_pass1=0 which should be fine since client respects the SessionResponse setting.

## Zone Capture File
scripts/capture-20260628_173650.log — FULL login+world+zone capture through proxy with port rewriting. This is the ground truth for comparing our bridge's zone packets.

## What's Next
1. Compare our bridge's PlayerProfile bytes against EQEmu's zone capture (need to decompress EQEmu data first since zone uses zlib)
2. The issue is likely in PlayerProfile struct field content, not the protocol layer — all protocol fixes are working correctly now
3. Also compare the Spawn struct (OP_ZoneEntry response) byte-for-byte

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Added OP_WorldComplete handler — zone connection now works end-to-end
> - Built outgoing packet fragmentation for packets exceeding max_packet_size=512
> - Added PlayerProfile CRC32 checksum and set encode_key=0 matching EQEmu
> - Created /runbridge skill and added zone port rewriting to UDP proxy
> - Achieved first full zone capture (login+world+zone) through proxy
>
> **Commits since last entry:**
> ```
> c5c6b26 chore(infra): add /runbridge skill for launching bridge in CMD window
> 8416f14 chore(server): add OP_WorldComplete to UDP proxy opcode dictionary
> 0ff8be6 feat(server): add WorldComplete handler, outgoing fragmentation, and PlayerProfile checksum
> ```
>
> **Files touched:**
> ```
> .claude/skills/runbridge/skill.md           |  27 +++
> scripts/udp-proxy.py                        |   1 +
> server/adif-bridge/src/main.rs              |  80 +++++++--
> server/adif-bridge/src/world_handler.rs     |  16 +-
> server/adif-bridge/src/titanium/opcodes.rs  |   2 +
> server/adif-bridge/src/titanium/structs.rs  |   8 +
> server/adif-bridge/src/eq_protocol/codec.rs |   2 +-
> ```
