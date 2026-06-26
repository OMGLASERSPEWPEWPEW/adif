## 2026-06-14_10-33-07 (session 1c33e3c8)

**assistant**: [tool: Edit]

**assistant**: Now the `.gitignore`, the reference README, and check the existing CLAUDE.md to add a Reference section:

**assistant**: [tool: Bash]

**assistant**: [tool: Write]

**assistant**: [tool: Write]

**assistant**: Now add a Reference section to CLAUDE.md. Let me read its current tail:

**assistant**: [tool: Read]

**assistant**: [tool: Edit]

**assistant**: Now final verification across all the changes:

**assistant**: [tool: Bash]

---

## 2026-06-21_15-44-33 (session c3ea3301)

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: PowerShell]

**assistant**: The `buzzing-skipping-koala.md` was the most recent one, from the EQMacEmu login fix — but that's an old issue. Let me look at the earlier session's journal more carefully. That session had an agent that did extensive reading and then wrote something. Let me check the later part of that journal.

**assistant**: [tool: Read]

**assistant**: I see Writes near the end of the session (15:29). Those look like they wrote memory files and updated the memory index. But then at the very end at 15:30-15:33 there were git commits happening. And earlier around 11:34-11:38 there was a burst of ~14 Write calls — that's likely where the document was created. Let me search for recently created files outside of reference/.

**assistant**: [tool: Glob]

**assistant**: [tool: PowerShell]

**assistant**: [tool: PowerShell]

**assistant**: OK so the postgresql-migration-progress.md was updated at 3:29 PM — I already read that. It has the detailed C++ conversion tracking. But you said "documentation on work left to be done" — let me look more carefully. The session also had a Write at 11:15 and around 11:34-11:38 that created migration files 009-022. And there were Writes at 15:29 updating memory. But you said "a few minutes ago" which would mean this current session or very close to it.

Could you point me to where you think i...

---

## 2026-06-22_08-39-41 (session 6d855ae2)

**assistant**: [tool: Read]

**assistant**: [tool: Edit]

**assistant**: [tool: Edit]

**assistant**: Now let me verify there are no remaining MySQL-isms in the custom repos.

**assistant**: [tool: Grep]

**assistant**: Only the comment remains — the actual SQL is all converted. Let me also check for remaining backticks.

**assistant**: [tool: PowerShell]

**assistant**: Phase 2 complete — zero MySQL-isms remaining in all custom repository files. Let me update the migration map.

**assistant**: [tool: Edit]

**assistant**: **Phase 2 done.** 4 phases left. Want to keep going with Phase 3 (zone server — ~10 `.cpp` files)?

---

## 2026-06-22_08-40-30 (session 6d855ae2)

**user**: let's /c&p what we have

**assistant**: [tool: Skill]

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
- Summary: imperative mood ("Add",...

**assistant**: Running the /cap snapshot.

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: **Phase 3: PROPOSE**

Proposed commits (3):

**1. `docs(database): update migration map with C++ conversion progress`**

> Files: docs/postgresql-migration-map.md

**2. `feat(database): add Python script for bulk MySQL→PG repo conversion`**

> Why: 250 auto-generated base repository files share identical MySQL patterns
> — a script converts REPLACE INTO, FROM_UNIXTIME, UNIX_TIMESTAMP, and backtick
> quoting to native PostgreSQL in one pass.
> Files: scripts/convert-repos-to-pg.py

**3. `chor...

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

## 2026-06-22_08-55-33 (session 6d855ae2)

**assistant**: [tool: Grep]

**assistant**: [tool: Grep]

**assistant**: Only comments remain — zero runtime MySQL SQL left. Now let's /cap everything.

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: Proposed commits (2):

**1. `docs(database): mark all C++ conversion phases complete in migration map`**

> Files: docs/postgresql-migration-map.md

**2. `chore(infra): update session journals and memory heaps`**

> Files: .claude/journals/*, .claude/memory/*

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: All 6 phases done and pushed. Here's the final tally:

| Phase | What | Files | Status |
|-------|------|-------|--------|
| 1 | Base repositories (script) | 244 | **DONE** |
| 2 | Custom repositories (manual) | 15 + 37 backtick-stripped | **DONE** |
| 3 | Zone server (parallel agents) | ~12 + 27 backtick-stripped | **DONE** |
| 4 | Common/world/login (parallel agents) | ~8 + 11 backtick-stripped | **DONE** |
| 5 | Perl generator + template | 2 | **DONE** |
| 6 | RewriteQuery() removed | 1 | **D...

---

## 2026-06-22_08-57-22 (session 6d855ae2)

**assistant**: Proposed commits (2):

**1. `docs(database): mark all C++ conversion phases complete in migration map`**

> Files: docs/postgresql-migration-map.md

**2. `chore(infra): update session journals and memory heaps`**

> Files: .claude/journals/*, .claude/memory/*

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: All 6 phases done and pushed. Here's the final tally:

| Phase | What | Files | Status |
|-------|------|-------|--------|
| 1 | Base repositories (script) | 244 | **DONE** |
| 2 | Custom repositories (manual) | 15 + 37 backtick-stripped | **DONE** |
| 3 | Zone server (parallel agents) | ~12 + 27 backtick-stripped | **DONE** |
| 4 | Common/world/login (parallel agents) | ~8 + 11 backtick-stripped | **DONE** |
| 5 | Perl generator + template | 2 | **DONE** |
| 6 | RewriteQuery() removed | 1 | **D...

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
- Summary: imperative mood ("Add",...

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: Just session tracking files. One commit:

**1. `chore(infra): update session journals and memory heaps`**

> Files: .claude/journals/*, .claude/memory/*

**Excluded:** `du.exe.stackdump`

Proceed? [yes / abort]

---

## 2026-06-22_09-05-23 (session 6d855ae2)

**assistant**: All 6 phases done and pushed. Here's the final tally:

| Phase | What | Files | Status |
|-------|------|-------|--------|
| 1 | Base repositories (script) | 244 | **DONE** |
| 2 | Custom repositories (manual) | 15 + 37 backtick-stripped | **DONE** |
| 3 | Zone server (parallel agents) | ~12 + 27 backtick-stripped | **DONE** |
| 4 | Common/world/login (parallel agents) | ~8 + 11 backtick-stripped | **DONE** |
| 5 | Perl generator + template | 2 | **DONE** |
| 6 | RewriteQuery() removed | 1 | **D...

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
- Summary: imperative mood ("Add",...

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: Just session tracking files. One commit:

**1. `chore(infra): update session journals and memory heaps`**

> Files: .claude/journals/*, .claude/memory/*

**Excluded:** `du.exe.stackdump`

Proceed? [yes / abort]

**user**: yes

**assistant**: [tool: Bash]

**assistant**: [tool: Bash]

**assistant**: Done. `25d3c48` pushed, working tree clean (just `du.exe.stackdump` remaining).

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

## 2026-06-24_09-02-16 (session e62794d9)

**assistant**: Now let me read the diffs to understand the content of each group.

**assistant**: [tools: Bash, Bash]

**assistant**: [tools: Bash, Bash, Bash]

**assistant**: [tools: Bash]

**assistant**: [tools: Bash, Bash]

**assistant**: Good. Now I have the full picture. Here's the analysis:

---

**Proposed commits (4):**

**1. `feat(database): add migrations 030-034 for EQEmu PostgreSQL schema alignment`**

Why: Character creation and zone boot required 9 table renames, 30+ missing tables, and dozens of column fixes to align ADIF's PostgreSQL schema with EQEmu's C++ expectations.
Files: database/migrations/030-034_*.sql

**2. `docs(database): add PostgreSQL migration error inventory`**

Files: docs/postgresql-errors-inventory.md

**3. `fix(infra): improve conversation logger with tool collapsing and system-reminder stripping`**

Why: Daily logs were bloated with raw system-reminder XML and individual tool-use lines; collapsing consecutive tools and stripping reminders makes logs readable.
Files: .claude/hooks/conversation-logger.sh

**4. `chore(infra): update session journals, memory heaps, and bridge journal`**

Files: .claude/bridge-journal.md, .claude/journals/*, .claude/memory/*

**Excluded:**
- `du.exe.stackdump` — crash dump, should not be committed
- `database/.claude/journals/` — session journal that ended up in the wrong place, should not be committed

Proceed? [yes / edit N / skip N / abort]

**user**: yes

**assistant**: Executing commits in order.

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: All 4 commits succeeded. Pushing now.

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: All clean. 4 commits pushed to main:

1. `18f51b9` **feat(database):** migrations 030-034 for EQEmu PostgreSQL schema alignment (1,101 lines)
2. `02cea27` **docs(database):** PostgreSQL migration error inventory
3. `60b6d63` **fix(infra):** conversation logger tool collapsing + system-reminder stripping
4. `c130bb1` **chore(infra):** session journals, memory heaps, bridge journal

Left uncommitted (intentionally): `du.exe.stackdump` and `database/.claude/` — neither belongs in the repo.

---

## 2026-06-25_19-16-09 (session 6b95b079)

**user**: yeah let's document each error, missing table, and column mismatch in a document first. Then let's /c&p after

**assistant**: [tools: Skill]

**user**: Base directory for this skill: E:\development\adif\.claude\skills\bridge

# /bridge — Founder's Bridge Journal

Append the user's message as a timestamped entry to `.claude/bridge-journal.md`, along with auto-gathered session context.

## What this is

The Bridge Journal is **the captain's voice** — not Claude's, not any agent's. It captures the user's thinking, priorities, frustrations, and decisions in their own words. Every agent reads it at session start as the highest-context source of truth. Each entry also carries auto-gathered session context (git history, conversation summary) so future agents understand what was happening when the captain spoke.

## Execution

1. Take the user's message (everything after `/bridge`)
2. Get the current timestamp in `YYYY-MM-DD HH:MM` format
3. **Gather git context:**
   - Read `.claude/bridge-journal.md` and find the most recent `### YYYY-MM-DD HH:MM` header
   - Run `git log --since="<last entry timestamp>" --oneline -20` (fall back to `--since="midnight"` if no prior entries)
   - Run `git diff --stat HEAD~10..HEAD`
4. **Summarize the conversation:**
   - Review the current conversation and distill 2-4 bullet points covering: what was built/fixed, key decisions, notable problems solved
   - Keep each bullet under 15 words
   - Omit this section entirely if the conversation was trivial (no substantive work before `/bridge`)
5. **Assemble the entry** using this format:

```markdown
### YYYY-MM-DD HH:MM

<user's message, verbatim>

> **Session context** *(auto-gathered)*
>
> **What happened:**
> - Built X with Y approach
> - Fixed Z caused by W
> - Decided to defer Q until next phase
>
> **Commits since last entry:**
> ```
> abc1234 feat(scope): short description
> def5678 fix(scope): another description
> ```
>
> **Files touched:**
> ```
> src/foo/bar.ts  | 42 +++--
> src/baz/qux.tsx | 28 ++-
> ```
```

6. Append to `.claude/bridge-journal.md`
7. Confirm with a single line: "Logged to the Bridge Journal."

## Rules

- **...

**assistant**: Let me gather the git context and write the bridge entry.

**assistant**: [tools: Bash]

**assistant**: [tools: Read]

**assistant**: [tools: Bash]

**assistant**: [tools: Edit]

**assistant**: Logged to the Bridge Journal.

Now let me document all the errors from the zone boot log into the error inventory.

**assistant**: [tools: Read]

**assistant**: [tools: Read]

**assistant**: Now let me write the updated error inventory with all the new errors from the zone boot log.

**assistant**: [tools: Write]

**assistant**: Error inventory updated. Here's the summary:

**Remaining work to clear all errors:**
- **22 missing tables** (mostly character subsystems: bandolier, potionbelt, disciplines, tasks, keyring, etc.)
- **12 column mismatches** (missing columns like `augslot1-6`, `close_timer_ms`, `taunting`; renamed columns like `slot` vs `slot_id`)
- **4 raw SQL column name mismatches** in C++ code (`charID` vs `char_id`, `p_accid` vs `accid`)
- **1 ON CONFLICT issue** on inventory table
- **1 reserved word** (`end` in base_data needs quoting)
- **4 uncommitted C++ fixes** from this session

Ready for `/cap` when you are.

**user**: <command-message>cap</command-message>
<command-name>/cap</command-name>
<command-args>and thank you for this!</command-args>

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
- Summary: imperative mood ("Add", "Fix", "Extract" — not "Added"...

---

## 2026-06-25_19-17-17 (session 6b95b079)

**assistant**: Logged to the Bridge Journal.

Now let me document all the errors from the zone boot log into the error inventory.

**assistant**: [tools: Read]

**assistant**: [tools: Read]

**assistant**: Now let me write the updated error inventory with all the new errors from the zone boot log.

**assistant**: [tools: Write]

**assistant**: Error inventory updated. Here's the summary:

**Remaining work to clear all errors:**
- **22 missing tables** (mostly character subsystems: bandolier, potionbelt, disciplines, tasks, keyring, etc.)
- **12 column mismatches** (missing columns like `augslot1-6`, `close_timer_ms`, `taunting`; renamed columns like `slot` vs `slot_id`)
- **4 raw SQL column name mismatches** in C++ code (`charID` vs `char_id`, `p_accid` vs `accid`)
- **1 ON CONFLICT issue** on inventory table
- **1 reserved word** (`end` in base_data needs quoting)
- **4 uncommitted C++ fixes** from this session

Ready for `/cap` when you are.

**user**: <command-message>cap</command-message>
<command-name>/cap</command-name>
<command-args>and thank you for this!</command-args>

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
| `perf` | Measurable performance improvement | `perf(filter): skip unchanged sentences on re-scan` |
| `style` | CSS/visual changes only | `style(ui): tighten palette carousel spacing` |
| `test` | Adding or fixing tests | `test(agendas): cover diffAgainstCache edge cases` |
| `docs` | Documentation only | `docs(claude-md): add agendas cache section` |
| `chore` | Agents, hooks, skills, config, deps | `chore(infra): add Thyra journal evolution entry` |

### Module scopes

| Scope | Files it covers |
|-------|----------------|
| `editor...

**assistant**: Let me gather the full picture of what changed.

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: The `du.exe.stackdump` is unrelated crash debris — should not be committed. `database/.claude/` is a database-specific Claude journal that can go with infra.

Proposed commits (3):

**1. `feat(database): add migrations 035-038 for composite PKs, starting items, color overflow, and remaining fixes`**
```
Why: Restore composite PKs after C++ ON CONFLICT fix, populate 148 starting 
     items from PEQ, fix BIGINT color overflow, add adventure/start_zones tables.
Files: 035_restore_composite_pks.sql, 036_starting_items_data.sql, 
       037_fix_color_column_overflow.sql, 038_remaining_fixes.sql
```

**2. `docs(database): update error inventory with zone entry errors and session 2026-06-25 fixes`**
```
Files: docs/postgresql-errors-inventory.m...

---

## 2026-06-25_19-18-27 (session 6b95b079)

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
| `perf` | Measurable performance improvement | `perf(filter): skip unchanged sentences on re-scan` |
| `style` | CSS/visual changes only | `style(ui): tighten palette carousel spacing` |
| `test` | Adding or fixing tests | `test(agendas): cover diffAgainstCache edge cases` |
| `docs` | Documentation only | `docs(claude-md): add agendas cache section` |
| `chore` | Agents, hooks, skills, config, deps | `chore(infra): add Thyra journal evolution entry` |

### Module scopes

| Scope | Files it covers |
|-------|----------------|
| `editor...

**assistant**: Let me gather the full picture of what changed.

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: The `du.exe.stackdump` is unrelated crash debris — should not be committed. `database/.claude/` is a database-specific Claude journal that can go with infra.

Proposed commits (3):

**1. `feat(database): add migrations 035-038 for composite PKs, starting items, color overflow, and remaining fixes`**
```
Why: Restore composite PKs after C++ ON CONFLICT fix, populate 148 starting 
     items from PEQ, fix BIGINT color overflow, add adventure/start_zones tables.
Files: 035_restore_composite_pks.sql, 036_starting_items_data.sql, 
       037_fix_color_column_overflow.sql, 038_remaining_fixes.sql
```

**2. `docs(database): update error inventory with zone entry errors and session 2026-06-25 fixes`**
```
Files: docs/postgresql-errors-inventory.md
```

**3. `chore(infra): update session journals, memory heaps, and bridge journal`**
```
Files: bridge-journal.md, journals/*, memory/daily/*, memory/heaps/*
```

Excluding: `du.exe.stackdump` (crash debris, not project code)

Proceed? [yes / edit N / skip N / abort]

**user**: yes

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: All 3 commits clean. Pushing now.

**assistant**: [tools: Bash]

**assistant**: [tools: Bash]

**assistant**: All pushed. 3 commits, 0 unpushed.

```
a89f328 feat(database): add migrations 035-038 for composite PKs, starting items, color overflow, and remaining fixes
4096a50 docs(database): update error inventory with zone entry errors and session 2026-06-25 fixes
fe9b4d2 chore(infra): update session journals, memory heaps, and bridge journal
```

And thank YOU — that was a marathon. We went from "zone unavailable" to standing in Grobb with 121 NPCs on PostgreSQL. The FindReplace infinite loop was a sneaky one but we got it. Next session: knock out those ~22 missing tables and ~12 column misma...

---

## 2026-06-25_20-30-06 (session 6b95b079)

**assistant**: [tools: Edit]

**assistant**: Now fix the inventory PrimaryKey:

**assistant**: [tools: Grep]

**assistant**: [tools: Edit]

**assistant**: [tools: Read]

**assistant**: [tools: Edit]

**assistant**: Now also fix the raw SQL query in zone_base_data.cpp:

**assistant**: [tools: Grep]

**assistant**: [tools: Grep]

**assistant**: The base_data raw SQL query must come from the repository (which we already fixed). Now rebuild:

**assistant**: [tools: PowerShell]

**assistant**: Built clean. Let me also rebuild world since it uses the inventory repository too:

**assistant**: [tools: PowerShell]

**assistant**: Both rebuilt. Now commit migration 042 and push:

**assistant**: [tools: Bash]

**assistant**: All pushed. Here's the session summary:

**What we fixed today:**
- Migrations 039-042: 22 missing tables created, ~15 columns fixed/renamed, 3 tables rebuilt
- C++ fixes: `base_data` column renames (`end` → `endurance`), inventory composite PK
- All cross-referenced against akk-stack MariaDB ground truth

**What's ready for tomorrow:**
- Zone transitions (need 2+ zone processes, and the `base_data`/`inventory` C++ fixes need a test)
- Remaining debug logging cleanup (all the `[HQS]`, `[HQSL]`, `[AddNPC]`, `[PZ-STEP]` cerr output)
- A few straggler column issues that may surface during combat/trading

Good stopping point. You're in Grobb on PostgreSQL with 121 NPCs walking around.

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
| `perf` | Measurable performance improvement | `perf(filter): skip unchanged sentences on re-scan` |
| `style` | CSS/visual changes only | `style(ui): tighten palette carousel spacing` |
| `test` | Adding or fixing tests | `test(agendas): cover diffAgainstCache edge cases` |
| `docs` | Documentation only | `docs(claude-md): add agendas cache section` |
| `chore` | Agents, hooks, skills, config, deps | `chore(infra): add Thyra journal evolution entry` |

### Module scopes

| Scope | Files it covers |
|-------|----------------|
| `editor...

**assistant**: [tools: Bash]

**assistant**: Just infra changes (journals + memory heaps) and the `du.exe.stackdump` (exclude).

Proposed commits (1):

**1. `chore(infra): update session journals and memory heaps`**
```
Files: journals/*, memory/daily/*, memory/heaps/*
```

Excluding: `du.exe.stackdump` (crash debris)

Proceed? [yes / abort]

---

