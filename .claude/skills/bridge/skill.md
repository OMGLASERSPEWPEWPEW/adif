---
name: bridge
description: >
  Write a timestamped entry to the Bridge Journal — the captain's personal
  log that every agent reads at session start. Use when the user wants to
  record a thought, priority, decision, or note for future sessions.
user_invocable: true
---

# /bridge — Founder's Bridge Journal

Append the user's message as a timestamped entry to `.claude/bridge-journal.md`, along with auto-gathered session context.

## What this is

The Bridge Journal is **the captain's voice** — not Claude's, not any agent's. It captures the user's thinking, priorities, frustrations, and decisions in their own words. Every agent reads it at session start as the highest-context source of truth. Each entry also carries auto-gathered session context (git history, build state) so future agents understand what was happening when the captain spoke.

## Execution

1. Take the user's message (everything after `/bridge`)
2. Get the current timestamp in `YYYY-MM-DD HH:MM` format
3. **Gather git context:**
   - Read `.claude/bridge-journal.md` and find the most recent `### YYYY-MM-DD HH:MM` header
   - Run `git log --since="<last entry timestamp>" --oneline -20` (fall back to `--since="midnight"` if no prior entries)
   - Run `git diff --stat HEAD~10..HEAD`
4. **Check build state:**
   - Note whether the project compiles and runs (from conversation context — don't re-run builds)
   - One line: e.g., `cargo build clean (25 warnings, 0 errors). Bridge runs. Client connects.`
5. **Check for redundancy:**
   - If the captain's entry already covers what happened in detail, **skip** the "What happened" summary in the auto-gathered block — just keep commits and files touched
   - Only write "What happened" bullets when the captain's entry is brief or omits work context
6. **Assemble the entry** using this format:

```markdown
### YYYY-MM-DD HH:MM `[Milestone Tag]`

<user's message, verbatim>

## Blockers / Open Questions
- [ ] Unresolved issue 1
- [ ] Unresolved issue 2
- [x] Previously open issue now resolved

> **Session context** *(auto-gathered)*
>
> **Build:** cargo build clean (N warnings, 0 errors). Bridge runs.
>
> **What happened:** *(only if captain's entry doesn't already cover it)*
> - Built X with Y approach
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

7. Append to `.claude/bridge-journal.md`
8. Confirm with a single line: "Logged to the Bridge Journal."

## Entry structure guidance

### Milestone tag

Tag each entry with the current milestone/phase in the header:

```
### 2026-06-29 19:30 `[M3: Protocol Bridge]`
```

Tags help when scanning the journal to find entries from a specific era. Use whatever tag fits — `[M1: PostgreSQL]`, `[M3: Combat]`, `[M4: Voxel]`, `[Client: Engine Selection]`, etc.

### Captain's entry: focus on WHY, not WHAT

The commit log already captures *what* changed. The captain's entry should focus on:
- **Why** we made the choices we did
- **Surprises** — things that didn't work as expected
- **Key discoveries** — hard-won technical knowledge that prevents re-learning
- **What's next** — this is the most valuable section; it directly drives the next session

Avoid changelog-style entries like "Added X, fixed Y, updated Z" — that's what the auto-gathered commits section is for.

### Blockers / Open Questions

A dedicated section for unresolved problems that span sessions. These persist until explicitly crossed off. Examples:

```markdown
## Blockers / Open Questions
- [ ] Reference EQEmu PG server: looting doesn't work (data chain OK, likely C++ runtime bug)
- [ ] Zone transitions: innothuleb (413) vs innothule (46) duplicate zone points — wrong spawn position
- [ ] CharSelect: character renders black — equipment/tint data zeroed
- [x] Camp/logout hangs — FIXED: added OP_Camp + OP_Logout handlers
```

When resolving a blocker from a previous entry, mark it `[x]` in the NEW entry (don't edit old entries). This creates a trail of when things were fixed.

### What's Next

The most-read section. Keep it as a numbered list of concrete next steps, ordered by priority. Future sessions start here.

## Rules

- **Never edit or paraphrase** the user's words. The conversation summary in the context block is the only part you write.
- **Never delete entries.** The journal is append-only.
- **Never write your own entries.** Only the user writes to the Bridge Journal.
- **Never mix your words with the captain's.** User message goes above the blockquote. Context goes inside the blockquote. They must never blend.
- **Keep context proportional.** The context block should never exceed ~15 lines. If there are 50+ commits, show the 10 most recent and note "... and N more."
- **Skip redundant summaries.** If the captain already wrote a detailed "What Happened", don't repeat it in the auto-gathered "What happened" bullets.
- If invoked with no message, ask: "What do you want to log?"

## Context block reference

Include each sub-section only when it has content:

| Sub-section | Include when |
|-------------|-------------|
| **Build** | Always — one line on compile/run state |
| **What happened** | Only if captain's entry is brief and omits work context |
| **Commits since last entry** | Any commits exist in the window |
| **Files touched** | Any files changed in the commit range |

If there is nothing to gather (fresh session, no commits, no prior work), the entry looks identical to the old format — just the user's message under the timestamp, no context block.

## Integration with other skills

- **`/standup`** — agents should reference recent bridge entries when arguing priorities; the Blockers section surfaces persistent issues
- **`/evolution`** — agents should check the bridge journal for founder context before reflecting
- **Session startup** — the bridge journal is item #2 on the orientation checklist, read before conversation journals. The `[Milestone Tag]` helps agents quickly find relevant entries.
- **All consumers** — treat blockquoted `> **Session context**` sections as supplementary machine context, not the captain's words. The Blockers section is action items — agents should check if their work resolves any.
