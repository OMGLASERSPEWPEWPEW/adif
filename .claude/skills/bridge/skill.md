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

- **Never edit or paraphrase** the user's words. The conversation summary in the context block is the only part you write.
- **Never delete entries.** The journal is append-only.
- **Never write your own entries.** Only the user writes to the Bridge Journal.
- **Never mix your words with the captain's.** User message goes above the blockquote. Context goes inside the blockquote. They must never blend.
- **Keep context proportional.** The context block should never exceed ~20 lines. If there are 50+ commits, show the 10 most recent and note "... and N more."
- If invoked with no message, ask: "What do you want to log?"

## Context block reference

Include each sub-section only when it has content:

| Sub-section | Include when |
|-------------|-------------|
| **What happened** | Conversation had substantive work before `/bridge` |
| **Commits since last entry** | Any commits exist in the window |
| **Files touched** | Any files changed in the commit range |

If there is nothing to gather (fresh session, no commits, no prior work), the entry looks identical to the old format — just the user's message under the timestamp, no context block.

## Integration with other skills

- **`/standup`** — agents should reference recent bridge entries when arguing priorities; the context blocks give richer information about what was accomplished
- **`/evolution`** — agents should check the bridge journal for founder context before reflecting
- **Session startup** — the bridge journal is item #2 on the orientation checklist, read before conversation journals
- **All consumers** — treat blockquoted `> **Session context**` sections as supplementary machine context, not the captain's words
