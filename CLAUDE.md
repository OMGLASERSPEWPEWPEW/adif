# ADIF — Another Day In Forever

An EverQuest-inspired MMORPG. Early-stage; engine and stack are not yet chosen.

## Status

Greenfield. No source code yet. This file and the `.claude/` tooling are the
first things in the repo. Update the **Stack** and **Architecture** sections
below once the engine/tech decisions are made.

## Stack

> TBD. Candidate directions to decide between:
> - **Game engine**: Godot (open-source, GDScript/C#), Unity (C#), or Unreal (C++).
> - **Web/3D**: Three.js or Babylon.js frontend + an authoritative game server.
> - **Backend/persistence**: authoritative server for world state, accounts,
>   inventory, combat. (The `blueprint` pattern library's React + Vite + Supabase
>   rules are available at `~/Development/patterns/blueprint/rules/` if a web
>   stack is chosen — install `stack.md`, `architecture.md`, `anti-patterns.md`
>   into `.claude/rules/` at that point.)

Once chosen, document the real stack here and remove this placeholder.

## Reference

`reference/eqmacemu/` holds a read-only clone of **EQMacEmu / The Al'Kabor
Project** (https://github.com/EQMacEmu/Server) — the open-source EverQuest server
emulator closest to the original classic→Planes-of-Power game. Use it to study
MMORPG server architecture: `world/` (world server), `zone/` (the simulation:
mobs/combat/spells/movement), `loginserver/`, `ucs/` (chat), `queryserv/`,
`common/` + `shared_memory/`, the DB schema, and Perl/Lua quest scripting.

The whole `reference/` folder is **gitignored** and **read-only** — don't edit it.
It's the GPLv3 server source only (no client/assets in the clone); supply your own
EQ Mac client to actually connect, and we'll build ADIF's own assets to replace
EQ's over time. See `reference/README.md`.

## Architecture

> TBD. MMORPG systems to plan for: zones/world streaming, authoritative
> server-side simulation, persistence, accounts/auth, networking/replication,
> combat, inventory/loot, NPCs/AI, chat. Define the directory layout here once
> the engine is picked.

## Claude Code Setup

This project is wired with the shared pattern libraries from
`~/Development/patterns/`:

- **`.claude/rules/`** — auto-loaded architecture norms.
  - `html-artifacts.md` — prefer HTML artifacts over markdown for complex
    analysis/specs/design explorations.
- **`.claude/hooks/`** — lifecycle automation (16 hooks; see
  `.claude/settings.json` for wiring). Includes context/cost tracking,
  git-push confirmation, ghostty status, completion notification, and
  session/SOTA sync hooks.
- **`.claude/skills/`** — 14 user-invoked workflows (`/standup`, `/new-feature`,
  `/docs-check`, `/escalate`, `/evolution`, `/retro`, etc.).
- **`.claude/agents/`** — the full `harbormoon` agent crew (25 agents across 8
  divisions: Command, Engineering, Quality, Design, Growth, Operations,
  Intelligence, Empathy). `divisions.json` defines the roster. Memory and
  standup scratch dirs live in `.claude/memory/` and `.claude/standups/`.

> **Configure before relying on the crew:**
> - `zephyr/agent.md` — fill in the PROJECT CONFIGURATION block (the lead/orchestrator).
> - `Theia/agent.md` — fill in the BRAND IDENTITY CONFIGURATION block.
>
> The agent-dependent hooks/skills (`orchestrator-init`, `/standup`, `/promote`,
> `/mind-meld`) are now active.
