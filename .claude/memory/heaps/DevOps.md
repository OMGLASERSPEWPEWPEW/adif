## 2026-06-14_10-18-13 (session 1c33e3c8)

**assistant**: [tool: Bash]

**assistant**: 16 hooks installed. Now all skills:

**assistant**: [tool: Bash]

**assistant**: Now the generic blueprint rule and a tailored CLAUDE.md for the game:

**assistant**: [tool: Bash]

**assistant**: Now a starter `CLAUDE.md` for the project. Since the stack/engine isn't chosen yet, I'll keep it focused on what the project is and what's been configured, without inventing a stack.

**assistant**: [tool: Write]

**assistant**: Let me verify the final layout and the merged settings:

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

---

## 2026-06-14_10-40-11 (session 1c33e3c8)

**assistant**: All done and verified.

## Done

**Agent config pre-filled** (placeholders gone — verified 0 remaining):
- `zephyr/agent.md` — lead orchestrator now knows: ADIF / EverQuest-like MMORPG, stack TBD, Phase 0 pre-production, points at the reference codebase.
- `Theia/agent.md` — draft brand identity (essence "Endless worlds, lasting bonds," voice, a fantasy color/font palette, and a terminology glossary). **These are first-draft values flagged for you to refine** — colors and fonts are placeholders....

**user**: <system-reminder>Message sent at Sun 2026-06-14 15:39:20 UTC.</system-reminder>
unnote it. This is a private passion project not a business.  getting the assets and stuff can't be wrong if twenty emulators haev it. We'll replace the assets but we need to get the game running first

**assistant**: Fair — it's your private project, and I overweighted the legal framing. Toning it down. One quick factual correction though, because it actually helps the "get it running" goal:

Emulators don't ship the assets precisely *because* redistribution isn't theirs to do — so every emulator setup has each user **supply their own client files**. "Twenty emulators have it" really means twenty users each brought their own copy. So the practical path to a running server is: grab the matching client yoursel...

**assistant**: [tool: Edit]

**assistant**: [tool: Edit]

**assistant**: Done. Quick check for any leftover legal phrasing:

**assistant**: [tool: Bash]

---

## 2026-06-22_07-23-35 (session 6d855ae2)

**assistant**: Now let me read the diffs and untracked files to understand the changes.

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: Let me check the remaining untracked files to decide what to include.

**assistant**: [tool: Read]

**assistant**: [tool: Bash]

**assistant**: **Phase 2: ANALYZE**

Here's what I see:

| File | Nature |
|------|--------|
| `docs/postgresql-migration-map.md` | New migration tracking doc |
| `.claude/hooks/session-journal.sh` + `.claude/settings.json` | Fix hook paths to use absolute paths (same fix) |
| `.claude/bridge-journal.md`, `.claude/journals/*`, `.claude/memory/daily/*`, `.claude/memory/heaps/*` | Auto-generated session/memory tracking |
| `du.exe.stackdump` | Crash dump — **exclude** (shouldn't be committed) |

---

**Phase 3...

---

## 2026-06-22_07-24-42 (session 6d855ae2)

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: All 3 committed. Pushing now.

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: All clean. 3 commits pushed to main:

- `989f55c` docs(database): add PostgreSQL migration map tracking 250 EQEmu tables
- `25f71a4` fix(infra): use absolute paths for session-journal hook
- `d975e43` chore(infra): add bridge journal, session journals, and memory heaps

Only `du.exe.stackdump` remains in the working tree (untracked) — you can delete it or add it to `.gitignore` whenever.

---

## 2026-06-22_07-27-11 (session 6d855ae2)

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: All 3 committed. Pushing now.

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: All clean. 3 commits pushed to main:

- `989f55c` docs(database): add PostgreSQL migration map tracking 250 EQEmu tables
- `25f71a4` fix(infra): use absolute paths for session-journal hook
- `d975e43` chore(infra): add bridge journal, session journals, and memory heaps

Only `du.exe.stackdump` remains in the working tree (untracked) — you can delete it or add it to `.gitignore` whenever.

**user**: Ok thank you. Let's talk about the migration map. What are we actually doing. Are we creating a view or are we changing the server's hardcoded mysql statements to postgres?

**assistant**: Both, actually — but at different layers.

**The database side** is mostly done. We've created ~129 PostgreSQL tables via migration files in `database/migrations/`. The remaining work there is the 146 missing tables and 9 name mismatches (like `zones` vs `zone`, `spells` vs `spells_new`). For the name mismatches, the simplest fix is just renaming our PG tables to match what the C++ expects — no views needed.

**The C++ side** is the bigger beast. The EQEmu server code at `reference/eqemu-ser...

---

## 2026-06-22_08-30-48 (session 6d855ae2)

**assistant**: [tool: Read]

**assistant**: That's clean. Let me also verify the timestamp-heavy account repo and the backtick-heavy character_data repo.

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: All conversions look correct:

- **UNIX_TIMESTAMP** → `EXTRACT(EPOCH FROM suspendeduntil)::int` (line 123)
- **FROM_UNIXTIME** → `TO_TIMESTAMP(...)` (line 306)
- **Backtick `class`** → `"class"` with PG double-quote quoting (line 169)
- **REPLACE INTO** → `INSERT INTO ... ON CONFLICT ... DO UPDATE SET` (verified earlier)

Now let me update the migration map to track this progress.

**assistant**: [tool: Read]

**assistant**: [tool: Edit]

**assistant**: Phase 1 is complete. Here's the summary:

**244 base repository files converted to native PostgreSQL:**
- 244 `REPLACE INTO` → `INSERT INTO ... ON CONFLICT (pk) DO UPDATE SET`
- 270 `FROM_UNIXTIME()` → `TO_TIMESTAMP()`
- 54 `UNIX_TIMESTAMP()` → `EXTRACT(EPOCH FROM ...)::int`
- 40 backtick identifiers → PostgreSQL double-quote identifiers
- `BaseUpsertSet()` method added to all 244 files for the ON CONFLICT SET clause

6 files were untouched (no MySQL-isms): bot_group_members, bot_groups,...

---

## 2026-06-22_08-45-08 (session 6d855ae2)

**assistant**: [tool: PowerShell]

**assistant**: [tool: Bash]

**assistant**: [tool: Grep]

**assistant**: [tool: Grep]

**assistant**: Backticks are stripped. Now let me do all the specific conversions. I'll use parallel agents for the zone files since they're independent.

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: Backtick stripping complete. 4 agents are working in parallel on the zone conversions — groups.cpp LIMIT removal, raids.cpp REPLACE INTO + LIMIT, mob/questmgr/task REPLACE INTO, and exp/client/zonedb/tradeskills misc MySQL patterns. Waiting for them to finish.

---

## 2026-06-22_08-45-22 (session 6d855ae2)

**assistant**: [tool: Grep]

**assistant**: [tool: Grep]

**assistant**: Backticks are stripped. Now let me do all the specific conversions. I'll use parallel agents for the zone files since they're independent.

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: Backtick stripping complete. 4 agents are working in parallel on the zone conversions — groups.cpp LIMIT removal, raids.cpp REPLACE INTO + LIMIT, mob/questmgr/task REPLACE INTO, and exp/client/zonedb/tradeskills misc MySQL patterns. Waiting for them to finish.

**user**: <task-notification>
<task-id>a63094306fcd74619</task-id>
<tool-use-id>toolu_01JDgVtuwkJ3cVvQ4FaMqKFr</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\a63094306fcd74619.output</output-file>
<status>completed</status>
<summary>Agent "Convert raids.cpp MySQL" came to rest</summary>
<note>A task-notification fires each time this agent comes to rest with no live background children of its own. The user can send it an...

**assistant**: Raids.cpp done — 2 REPLACE INTO converted, 4 UPDATE LIMIT removed. Waiting on the other 3 agents.

---

## 2026-06-22_08-45-30 (session 6d855ae2)

**assistant**: Backticks are stripped. Now let me do all the specific conversions. I'll use parallel agents for the zone files since they're independent.

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: Backtick stripping complete. 4 agents are working in parallel on the zone conversions — groups.cpp LIMIT removal, raids.cpp REPLACE INTO + LIMIT, mob/questmgr/task REPLACE INTO, and exp/client/zonedb/tradeskills misc MySQL patterns. Waiting for them to finish.

**user**: <task-notification>
<task-id>a63094306fcd74619</task-id>
<tool-use-id>toolu_01JDgVtuwkJ3cVvQ4FaMqKFr</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\a63094306fcd74619.output</output-file>
<status>completed</status>
<summary>Agent "Convert raids.cpp MySQL" came to rest</summary>
<note>A task-notification fires each time this agent comes to rest with no live background children of its own. The user can send it an...

**assistant**: Raids.cpp done — 2 REPLACE INTO converted, 4 UPDATE LIMIT removed. Waiting on the other 3 agents.

**user**: <task-notification>
<task-id>a53e03b9d90c3ec3a</task-id>
<tool-use-id>toolu_01AhLQ3yA9jN7TVm1zFQqANn</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\a53e03b9d90c3ec3a.output</output-file>
<status>completed</status>
<summary>Agent "Convert groups.cpp LIMIT" came to rest</summary>
<note>A task-notification fires each time this agent comes to rest with no live background children of its own. The user can send it a...

**assistant**: Groups.cpp done — 10 UPDATE LIMIT 1 removed. 2 agents still working on the REPLACE INTO and misc conversions.

---

## 2026-06-22_08-45-55 (session 6d855ae2)

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: Backtick stripping complete. 4 agents are working in parallel on the zone conversions — groups.cpp LIMIT removal, raids.cpp REPLACE INTO + LIMIT, mob/questmgr/task REPLACE INTO, and exp/client/zonedb/tradeskills misc MySQL patterns. Waiting for them to finish.

**user**: <task-notification>
<task-id>a63094306fcd74619</task-id>
<tool-use-id>toolu_01JDgVtuwkJ3cVvQ4FaMqKFr</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\a63094306fcd74619.output</output-file>
<status>completed</status>
<summary>Agent "Convert raids.cpp MySQL" came to rest</summary>
<note>A task-notification fires each time this agent comes to rest with no live background children of its own. The user can send it an...

**assistant**: Raids.cpp done — 2 REPLACE INTO converted, 4 UPDATE LIMIT removed. Waiting on the other 3 agents.

**user**: <task-notification>
<task-id>a53e03b9d90c3ec3a</task-id>
<tool-use-id>toolu_01AhLQ3yA9jN7TVm1zFQqANn</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\a53e03b9d90c3ec3a.output</output-file>
<status>completed</status>
<summary>Agent "Convert groups.cpp LIMIT" came to rest</summary>
<note>A task-notification fires each time this agent comes to rest with no live background children of its own. The user can send it a...

**assistant**: Groups.cpp done — 10 UPDATE LIMIT 1 removed. 2 agents still working on the REPLACE INTO and misc conversions.

**user**: <task-notification>
<task-id>adb52c221f248b27c</task-id>
<tool-use-id>toolu_018npjCypaDXbQLEnxKFn8Cz</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\adb52c221f248b27c.output</output-file>
<status>completed</status>
<summary>Agent "Convert zone misc MySQL" came ...

---

## 2026-06-22_08-48-13 (session 6d855ae2)

**assistant**: [tool: Grep]

**assistant**: [tool: Grep]

**assistant**: [tool: Grep]

**assistant**: [tool: Grep]

**assistant**: World and login are clean already. Phase 4 is just `common/*.cpp` files. Let me strip backticks first, then convert the 13 specific patterns.

**assistant**: [tool: PowerShell]

**assistant**: Now let me do the actual SQL conversions with parallel agents.

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: Two agents working on Phase 4 — database.cpp (6 conversions) and the remaining common files (7 conversions). Waiting for them to finish.

Note: The `database_update_manifest.h` files contain ~70 `SHOW TABLES LIKE` checks and some `REPLACE INTO`/`GROUP_CONCAT` in embedded DDL migration scripts. These are MySQL migration manifests that won't apply to our PostgreSQL setup (we have our own `database/migrations/`), so I'm leaving them as-is. They'll become dead code once we remove the RewriteQuery ...

---

## 2026-06-22_08-48-38 (session 6d855ae2)

**assistant**: [tool: Grep]

**assistant**: [tool: Grep]

**assistant**: World and login are clean already. Phase 4 is just `common/*.cpp` files. Let me strip backticks first, then convert the 13 specific patterns.

**assistant**: [tool: PowerShell]

**assistant**: Now let me do the actual SQL conversions with parallel agents.

**assistant**: [tool: Agent]

**assistant**: [tool: Agent]

**assistant**: Two agents working on Phase 4 — database.cpp (6 conversions) and the remaining common files (7 conversions). Waiting for them to finish.

Note: The `database_update_manifest.h` files contain ~70 `SHOW TABLES LIKE` checks and some `REPLACE INTO`/`GROUP_CONCAT` in embedded DDL migration scripts. These are MySQL migration manifests that won't apply to our PostgreSQL setup (we have our own `database/migrations/`), so I'm leaving them as-is. They'll become dead code once we remove the RewriteQuery ...

**user**: <task-notification>
<task-id>a7c72629ec45222b7</task-id>
<tool-use-id>toolu_01Q7euFmqS7QyEbPBpeSwviA</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\a7c72629ec45222b7.output</output-file>
<status>completed</status>
<summary>Agent "Convert database.cpp MySQL" came to rest</summary>
<note>A task-notification fires each time this agent comes to rest with no live background children of its own. The user can send it...

**assistant**: database.cpp done — 6 conversions. Waiting on the common misc agent.

---

## 2026-06-22_08-49-18 (session 6d855ae2)

**assistant**: [tool: Agent]

**assistant**: Two agents working on Phase 4 — database.cpp (6 conversions) and the remaining common files (7 conversions). Waiting for them to finish.

Note: The `database_update_manifest.h` files contain ~70 `SHOW TABLES LIKE` checks and some `REPLACE INTO`/`GROUP_CONCAT` in embedded DDL migration scripts. These are MySQL migration manifests that won't apply to our PostgreSQL setup (we have our own `database/migrations/`), so I'm leaving them as-is. They'll become dead code once we remove the RewriteQuery ...

**user**: <task-notification>
<task-id>a7c72629ec45222b7</task-id>
<tool-use-id>toolu_01Q7euFmqS7QyEbPBpeSwviA</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\a7c72629ec45222b7.output</output-file>
<status>completed</status>
<summary>Agent "Convert database.cpp MySQL" came to rest</summary>
<note>A task-notification fires each time this agent comes to rest with no live background children of its own. The user can send it...

**assistant**: database.cpp done — 6 conversions. Waiting on the common misc agent.

**user**: <task-notification>
<task-id>a2de025e5277399d4</task-id>
<tool-use-id>toolu_01YXEkqJTK3yeEFDNX7nciPM</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\a2de025e5277399d4.output</output-file>
<status>completed</status>
<summary>Agent "Convert common misc MySQL" came to rest</summary>
<note>A task-notification fires each time this agent comes to rest with no live background children of its own. The user can send it ...

**assistant**: Phase 4 complete. Let me verify and update the migration map.

**assistant**: [tool: Grep]

**assistant**: Only `dbcore.cpp` remains — that's the `RewriteQuery()` layer itself, which we'll gut in Phase 6. All runtime SQL in common/*.cpp is now native PostgreSQL.

**assistant**: [tool: Edit]

**assistant**: **Phases...

---

## 2026-06-22_10-04-53 (session 6d855ae2)

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: Now let me check what columns our PG table actually has, and add the missing ones.

**assistant**: [tool: PowerShell]

**assistant**: The table needs a full rebuild — column names don't match at all. Let me drop and recreate it with the exact schema EQEmu expects, plus fix the other broken tables. I'll run it directly against PG.

**assistant**: [tool: PowerShell]

**assistant**: Almost clean — just the `account_ip` constraint syntax (PG doesn't support `IF NOT EXISTS` on `ADD CONSTRAINT`). Let me fix that.

**assistant**: [tool: PowerShell]

**assistant**: [tool: PowerShell]

**assistant**: All fixes applied:
- `character_data` — rebuilt with full 106-column EQEmu schema
- `name_filter` — added `id` column
- `data_buckets` — added missing columns (account_id, character_id, etc.)
- `account_ip` — added unique constraint on (accid, ip)
- `character_pet_name` — created

Try creating a character again — it should get past name approval and the INSERT should work now. The servers are still running with the old binaries (which already have the fixed SQL), so just try in the c...

---

## 2026-06-22_10-08-44 (session 6d855ae2)

**assistant**: Almost clean — just the `account_ip` constraint syntax (PG doesn't support `IF NOT EXISTS` on `ADD CONSTRAINT`). Let me fix that.

**assistant**: [tool: PowerShell]

**assistant**: [tool: PowerShell]

**assistant**: All fixes applied:
- `character_data` — rebuilt with full 106-column EQEmu schema
- `name_filter` — added `id` column
- `data_buckets` — added missing columns (account_id, character_id, etc.)
- `account_ip` — added unique constraint on (accid, ip)
- `character_pet_name` — created

Try creating a character again — it should get past name approval and the INSERT should work now. The servers are still running with the old binaries (which already have the fixed SQL), so just try in the c...

**user**: E 1: ...T id FROM character_data WHERE account_id = 1 AND deleted_at...
                                                             ^
] Query [SELECT id FROM character_data WHERE account_id = 1 AND deleted_at IS NULL]
 World |    Info    | operator () New [EverQuest Client] connection from IP [127.0.0.1:50913]
 World |    Info    | operator () New connection from [127.0.0.1:50913,] processing connection
 World |    Info    | HandleSendLoginInfoPacket Account (darklight) Logging (in) to characte...

**user**: E 1: ...T id FROM character_data WHERE account_id = 1 AND deleted_at...
                                                             ^
] Query [SELECT id FROM character_data WHERE account_id = 1 AND deleted_at IS NULL]
 World |    Info    | operator () New [EverQuest Client] connection from IP [127.0.0.1:50913]
 World |    Info    | operator () New connection from [127.0.0.1:50913,] processing connection
 World |    Info    | HandleSendLoginInfoPacket Account (darklight) Logging (in) to characte...

**assistant**: The `RestTimer` error is a PostgreSQL case-sensitivity issue — the column was created as `"RestTimer"` (quoted, preserving case) but queries reference it unquoted, so PG looks for `resttimer` (lowercase). Quick fix ...

---

## 2026-06-24_08-52-03 (session e62794d9)

**user**: hello can you look at our journals and git and stuff where are we at

**assistant**: Let me pull up the bridge journal, recent sessions, and git state to give you a full recap.

**assistant**: [tools: Read, Bash]

**assistant**: [tools: Read]

**assistant**: [tools: Read]

**assistant**: Let me also check the TODOs and the error inventory from last session.

**assistant**: [tools: Bash, Read]

**assistant**: [tools: Read]

**assistant**: ---

## ZEPHYR RECAP

**The big picture: you've been on a MySQL-to-PostgreSQL conversion marathon for the EQEmu reference server, and you're tantalizingly close to zoning in.**

Over the last 3 sessions (June 21-23), you went from a crash-looping MariaDB to a fully PostgreSQL-backed EQEmu server. The arc:

1. **June 21** — Fixed MariaDB, stood up PG-backed server, got to character select for the first time on PostgreSQL. Hit a wall of missing/mismatched table schemas.

2. **June 22** — The big one. Converted **290+ C++ source files** from MySQL SQL to native PostgreSQL. Bulk conversion script for 250 base repos, manual work on 27 custom repos, parallel agents on zone/world files. Gutted the `RewriteQuery()` runtime shim. First successful server boot on PG — 618 zones loaded.

3. **June 23** — Applied migrations 030-034, fixed 13 blockers (name rejection, column mismatches, missing tables, quest plugin stubs). **Character creation works. Zone boots with maps/water/navmesh.** But zone entry itself times out — the last blocker is **ON CONFLICT composite PK mismatches** in character_bind, character_skills, character_languages, and rule_values. The C++ uses `ON CONFLICT (id)` but these tables have composite PKs like `(id, slot)`.

**What's still broken (documented in `docs/postgresql-errors-inventory.md`):**
- 5 ON CONFLICT composite PK fixes needed in C++ custom repos
- 5 column name mismatches to fix
- 3 missing tables (spawn2_disabled, global_loot, ldon_trap_*)
- 8 hardcoded MySQL syntax remnants in C++ (backticks, REGEXP, LIMIT)
- ~25 non-fatal missing tables (Tier 2 gameplay systems)

**None of these changes are committed** — the migrations (030-034), C++ edits, quest stubs, and DB triggers are all uncommitted. That's a lot of work sitting in the working tree.

**The immediate next step** per the bridge journal: fix those ON CONFLICT composite PK targets in the C++ custom repos, rebuild, and test zone entry end-to-end. That's the last gate before you can...

**user**: <command-message>cap</command-message>
<command-name>/cap</command-name>

**user**: Base directory for this skill: E:\development\adif\.claude\skills\cap

# /c&p — Commit and Push

Group changes into logical commits with machine-readable messages, then push.

## Commit Convention

### Message format

```
type(scope): imperative summary (72 chars max)

Why: one or two sentences explaining reasoning, not the diff
Files: key-file-1.ts, key-file-2.ts, key-file-3.ts

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Rules:**
- Summary: imperative mood ("Add", "Fix", "Extract" — not "Added", "Fixing"), 72 chars max
- `Why:` trailer: required for all types except `chore` and `docs`
- `Files:` trailer: required, list the 2-5 most significant files touched
- `Co-Authored-By:` footer: always present
- Multi-scope: if genuinely equal, list both: `feat(palette,shared): extract helper`

### Exempt types

`Evolve:` and `Promote:` are ritual commit types produced by `/evolution` and `/promote`. They use their own format. **Never rewrite, absorb, or duplicate these.** If detected in today's git log, exclude their files from grouping.

### Change types

| Type | When to use | Example |
|------|------------|---------|
| `feat` | New user-facing capability | `feat(palette): add per-pane model selection` |
| `fix` | Corrects a bug | `fix(agendas): prevent stale cache on day rollover` |
| `refactor` | Restructuring without behavior change | `refactor(palette): extract pane registry` |
|...

---

