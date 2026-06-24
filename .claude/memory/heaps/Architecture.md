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

## 2026-06-22_09-18-01 (session 6d855ae2)

**assistant**: [tool: Bash]

**assistant**: Done. `25d3c48` pushed, working tree clean (just `du.exe.stackdump` remaining).

**user**: okay let's give it a try

**assistant**: Hold on — we modified the C++ source files, but the compiled binaries in `reference/eqemu-server/build/bin/RelWithDebInfo/` are still from the old build (with MySQL SQL + RewriteQuery). We need to rebuild the server for our changes to take effect.

Want me to kick off a rebuild? The build was done with CMake previously — let me check what's set up.

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Grep]

**assistant**: Build system is ready with PostgreSQL configured. Let me kick off the rebuild.

**assistant**: [tool: Bash]

**assistant**: Build is running in the background (up to 10 min timeout). It'll recompile the ~290 modified files and relink all server binaries. I'll let you know as soon as it finishes.

---

## 2026-06-22_09-18-22 (session 6d855ae2)

**assistant**: [tool: Bash]

**assistant**: [tool: Grep]

**assistant**: Build system is ready with PostgreSQL configured. Let me kick off the rebuild.

**assistant**: [tool: Bash]

**assistant**: Build is running in the background (up to 10 min timeout). It'll recompile the ~290 modified files and relink all server binaries. I'll let you know as soon as it finishes.

**user**: <task-notification>
<task-id>b4g1a9ds3</task-id>
<tool-use-id>toolu_01YG6XCZ277vyrYbf64BZoAU</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\b4g1a9ds3.output</output-file>
<status>completed</status>
<summary>Background command "Rebuild EQEmu server with PostgreSQL changes" completed (exit code 0)</summary>
</task-notification>

**assistant**: [tool: Read]

**assistant**: CMake isn't in the bash PATH. Let me use PowerShell instead.

**assistant**: [tool: PowerShell]

**assistant**: Build running via PowerShell now. This will take a few minutes to recompile the modified files and relink. I'll let you know when it's done.

---

## 2026-06-22_09-18-55 (session 6d855ae2)

**assistant**: [tool: Read]

**assistant**: CMake isn't in the bash PATH. Let me use PowerShell instead.

**assistant**: [tool: PowerShell]

**assistant**: Build running via PowerShell now. This will take a few minutes to recompile the modified files and relink. I'll let you know when it's done.

**user**: <task-notification>
<task-id>bg2ia208j</task-id>
<tool-use-id>toolu_01PahJVjiKtFW5K1dBwGiNoF</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\bg2ia208j.output</output-file>
<status>failed</status>
<summary>Background command "Rebuild EQEmu server with PostgreSQL changes" failed with exit code 1</summary>
</task-notification>

**assistant**: [tool: Read]

**assistant**: CMake isn't in the system PATH. Let me find it.

**assistant**: [tool: PowerShell]

**assistant**: [tool: PowerShell]

**assistant**: Found CMake inside Visual Studio 18. Build is running now — this one should work. Will report back when it finishes.

---

## 2026-06-22_09-20-23 (session 6d855ae2)

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: Two compile errors — both `empty character constant` which means backtick stripping removed backticks that were used as actual C++ character literals (not SQL). Let me fix them.

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: The blanket backtick strip hit two C++ character literals that were checking for actual backtick characters in strings (not SQL). Let me restore them.

**assistant**: [tool: Edit]

**assistant**: [tool: Edit]

**assistant**: [tool: PowerShell]

**assistant**: Fixed the two false positives — backtick characters that were C++ char literals, not SQL quoting. Rebuilding now.

---

## 2026-06-22_09-25-04 (session 6d855ae2)

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: The blanket backtick strip hit two C++ character literals that were checking for actual backtick characters in strings (not SQL). Let me restore them.

**assistant**: [tool: Edit]

**assistant**: [tool: Edit]

**assistant**: [tool: PowerShell]

**assistant**: Fixed the two false positives — backtick characters that were C++ char literals, not SQL quoting. Rebuilding now.

**user**: <task-notification>
<task-id>bz5qbawti</task-id>
<tool-use-id>toolu_01DbF65zSX2h6nrWB1tnniKt</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\bz5qbawti.output</output-file>
<status>completed</status>
<summary>Background command "Rebuild after fixing char literal errors" completed (exit code 0)</summary>
</task-notification>

**assistant**: [tool: Read]

**assistant**: Build succeeded — all binaries compiled clean. `zone.exe`, `world.exe`, `loginserver.exe`, `shared_memory.exe` are all rebuilt with native PostgreSQL SQL.

Ready to start the servers. From the bridge journal, the startup order is:

1. `shared_memory.exe` — loads spells, items, NPCs
2. `loginserver.exe` — account auth
3. `world.exe` — world server + zone launcher

Want me to fire them up, or do you want to run them manually? (They're at `reference/eqemu-server/build/bin/RelWithDebInfo/`)

---

## 2026-06-22_10-01-34 (session 6d855ae2)

**user**: <task-notification>
<task-id>b1vrfh1l4</task-id>
<tool-use-id>toolu_01TTPcpfw7oybzvycVLcCPgL</tool-use-id>
<output-file>C:\Users\OMGLAS~1\AppData\Local\Temp\claude\E--development-adif\6d855ae2-6046-4d80-8b40-48298c9d8ee9\tasks\b1vrfh1l4.output</output-file>
<status>completed</status>
<summary>Background command "Rebuild after stopping servers" completed (exit code 0)</summary>
</task-notification>

**assistant**: [tool: Read]

**assistant**: All binaries built clean — world, zone, loginserver, shared_memory all linked. Starting servers.

**assistant**: [tool: PowerShell]

**assistant**: [tool: PowerShell]

**assistant**: All three servers are up with the INTERVAL + composite key fixes. Try logging in again — the INTERVAL errors and the rule_values ON CONFLICT errors should be gone. The missing tables and character_data column mismatches will still show, but those are the migration work for next session.

**user**: did we /c&p

**assistant**: Not yet — let me do that now.

**assistant**: [tool: Bash]

**assistant**: Proposed commits (2):

**1. `docs(infra): update bridge journal with C++ conversion session recap`**

> Files: .claude/bridge-journal.md

**2. `chore(infra): update session journals and memory heaps`**

> Files: .claude/journals/*, .claude/memory/*

Proceed? [yes / abort]

---

## 2026-06-22_10-02-16 (session 6d855ae2)

**assistant**: All three servers are up with the INTERVAL + composite key fixes. Try logging in again — the INTERVAL errors and the rule_values ON CONFLICT errors should be gone. The missing tables and character_data column mismatches will still show, but those are the migration work for next session.

**user**: did we /c&p

**assistant**: Not yet — let me do that now.

**assistant**: [tool: Bash]

**assistant**: Proposed commits (2):

**1. `docs(infra): update bridge journal with C++ conversion session recap`**

> Files: .claude/bridge-journal.md

**2. `chore(infra): update session journals and memory heaps`**

> Files: .claude/journals/*, .claude/memory/*

Proceed? [yes / abort]

**user**: yes

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: Done. 2 commits pushed:

- `3a05b13` docs(infra): update bridge journal with C++ conversion session recap
- `43d4f2c` chore(infra): update session journals and memory heaps

Massive session today — 290+ C++ files converted from MySQL to native PostgreSQL, server boots and connects. Next session: migration files to fix column mismatches and create the ~30 missing tables.

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

## 2026-06-23_08-53-20 (session 6ac4a7ee)

**user**: hello where did we leave off?

**assistant**: Let me pull up the session context and get you oriented.

**assistant**: [tool: Read]

**assistant**: [tool: Bash]

**assistant**: [tool: Read]

**assistant**: [tool: Glob]

**assistant**: [tool: Read]

**assistant**: ## ZEPHYR RECAP

**The big picture: You're converting the EQEmu server from MySQL to PostgreSQL, and it's going incredibly well.**

Over the last two sessions, you knocked out a massive amount of work:

**Session 1 (June 21)** — Fixed MariaDB crash-loops, set up the PostgreSQL-backed EQEmu build, fought through config issues (port types, account ID=0), and got your **first successful PostgreSQL login** — character select screen appeared. Character creation was blocked by schema mismatches.

...

---

## 2026-06-23_08-55-59 (session 6ac4a7ee)

**assistant**: [tool: Read]

**assistant**: [tool: Bash]

**assistant**: [tool: Glob]

**assistant**: [tool: Read]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: The session ended with an `ExitPlanMode` — so you created a plan at the very end. Let me check the memory heaps for what was captured.

**assistant**: [tool: Read]

**assistant**: [tool: Read]

**assistant**: NOW I have the full picture. Here's where we actually left off:

**You were trying to log into the server after rebuilding with the PostgreSQL-native C++ code.** You pasted the server errors into the chat. The errors were:

1. **`column "account_id" does not exist`** on the `data_buckets` table — we fixed that live by adding the missing columns
2. **`RestTimer` case sensitivity** — PostgreSQL treated the quoted `"RestTimer"` as case-sensitive, but the C++ queries reference it unquoted (which...

---

## 2026-06-23_09-10-48 (session 6ac4a7ee)

**assistant**: That clears it up. The session journal (`session-journal.sh`) is just a lightweight tool-call event log — timestamps and tool names, not actual conversations. The **conversation content** comes from `conversation-logger.sh`, which fires on the `Stop` event and parses the `.jsonl` transcript file.

Let me check exactly how the conversation logger is wired and whether it's actually working.

**assistant**: [tools: Read]

**assistant**: [tools: Grep]

**assistant**: [tools: Grep]

**assistant**: Now I have the full picture. Here's how it actually works:

**You have TWO separate logging systems:**

| System | Hook File | Fires On | Captures |
|--------|-----------|----------|----------|
| **Session Journal** | `session-journal.sh` | SessionStart, PreToolUse, PostToolUse, Notification, Stop | **Metadata only** — timestamps, tool names, state transitions |
| **Conversation Logger** | `conversation-logger.sh` | Stop only | **Actual conversation text** — user messages, assistant responses, tool call stubs |

The **conversation logger IS working** — that's where all those conversation snippets in `.claude/memory/daily/2026-06-22.md` and `.claude/memory/heaps/Database.md` came from. It parses the `.jsonl` transcript on each `Stop` event and writes:
- Last 20 snippets (truncated to 500 chars each) → daily log
- Topic-matched snippets → heap files (Database, Architecture, Auth, etc.)

The **session journal** (`.claude/journals/<id>.md`) is intentionally just a lightweight event log — it tells you *when* tools were called, not *what* was said. Think of it as an activity timeline for debugging.

So the conversation capture IS happening, just through the conversation-logger, not the session journal.

**assistant**: [tools: AskUserQuestion]

**user**: [Request interrupted by user for tool use]

**user**: name approval and the INSERT should work now. The servers are still running with the old binaries (which already have the fixed SQL), so just try in the c...

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

**assistant**: The `RestTimer` error is a PostgreSQL case-sensitivity issue — the column was created as `"RestTimer"` (quoted, preserving case) but queries reference it unquoted, so PG looks for `resttimer` (lowercase). Quick fix for next session: rename the column to lowercase.

Let me document this and exit.

**assistant**: [tool: Edit]

**assistant**: [tool: ExitPlanMode]

**assistant**: All documented. For next session, the first thing to do is:

```sql
ALTER TABLE character_data RENAME COLUMN "RestTimer" TO resttimer;
```

That's it — one line fixes the case sensitivity, then character select + creation should work. After that, we create the ~30 missing tables via migration files.

Huge session today — 290+ files converted, server booting on PostgreSQL, and we're one lowercase column renam...

**assistant**: Good point — looking at the conversation logger code, the problem is clear. Let me trace exactly why your words get lost.

**assi...

---

## 2026-06-23_17-51-09 (session 4ce700a5)

**user**: hello where we at

**assistant**: Let me pull up the latest state of things.

**assistant**: [tools: Read, Bash]

**assistant**: [tools: Read]

**assistant**: [tools: Read]

**assistant**: [tools: Read]

**assistant**: [tools: Glob]

**assistant**: Let me check the migration map and TODO tracker for current state.

**assistant**: [tools: Read]

**assistant**: ## ZEPHYR RECAP

**The big news: the C++ conversion is done.** All 290+ source files across the EQEmu codebase have been converted from MySQL to native PostgreSQL SQL. That was the monster — `REPLACE INTO`, `FROM_UNIXTIME`, backtick quoting, `ON DUPLICATE KEY UPDATE`, `IFNULL` — all of it, across base repositories, custom repositories, zone server, common, world, and login. The Perl generator and template were also fixed so future generated code comes out PG-native. The `RewriteQuery()` runtime translation layer was gutted down to a passthrough. First successful build and server boot on PostgreSQL happened yesterday — login works, character select loads, 618 zones recognized.

**What's blocking progress:** Character creation. The server boots and you can log in and see the character list, but creating a new character fails because:
1. **9 tables need renaming** — EQEmu expects `zone` but PG has `zones`, `npc_types` vs `npc_templates`, `spells_new` vs `spells`, etc.
2. **`character_data` needs rebuilding** — 51 columns present, 106 expected
3. **Column mismatches** on `doors` (missing 4), `guilds` (missing 5), `guild_members` (missing 1)
4. **~17 Tier 1 tables are missing** entirely (AA system, `base_data`, `faction_base_data`, etc.)
5. **Name filter table** is broken (missing `id` column)

**What's on the horizon:** The migration map lays out the path — Phase 1 (renames), Phase 2 (schema fixes), Phase 3 (missing critical tables). Each phase ends with a server restart to test how much further we get. After that, ~80 Tier 2 gameplay tables. The C++ is done; this is all SQL migration work now.

No active TODOs in `docs/todo/`. The bridge journal and migration map are the roadmap.

What would you like to tackle?

---

## 2026-06-24_08-50-47 (session e62794d9)

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

