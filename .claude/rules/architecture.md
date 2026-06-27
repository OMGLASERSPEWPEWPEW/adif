# Project Architecture

## Directory Layout

```
proto/
  buf.yaml                  # Module config + lint/breaking rules
  buf.gen.yaml              # Code generation targets
  image.bin                 # Breaking-change baseline (buf build output)
  README.md                 # Protocol guide
  adif/                     # All .proto files (one per domain)
    packet.proto            # Client-server envelope (173-message oneof)
    ipc.proto               # Server-to-server envelope (77-message oneof)
    common.proto            # Shared types (Vec3, Color)
    <domain>.proto          # Domain messages (zone, entity, combat, ...)

database/
  docker-compose.yml        # PostgreSQL + Redis containers
  migrations/               # Numbered SQL files (001_accounts.sql, ...)

src/                        # Server source code
  Common/                   # Shared library (protocol, DB access, config)
  LoginServer/              # Login + auth process
  WorldServer/              # Character select, zone routing, cross-zone coord
  ZoneServer/               # Per-zone game logic (combat, AI, spells, movement)
  ChatServer/               # Cross-zone chat/mail

tests/                      # Verification projects
scripts/                    # Dev automation (PowerShell)
docs/                       # Analysis artifacts (HTML preferred per rules)
reference/                  # Gitignored upstream study material
```

## Decision Framework

- "Does it define a wire message?" -> `proto/adif/`
- "Does it change persistent schema?" -> `database/migrations/` (new numbered file)
- "Does it need to run in every server process?" -> `src/Common/`
- "Is it zone-specific game logic (combat, AI, spawns)?" -> `src/ZoneServer/`
- "Is it cross-zone coordination?" -> `src/WorldServer/`
- "Is it a dev tool or setup script?" -> `scripts/`
- "Is it analysis, comparison, or exploration?" -> `docs/` as HTML

## Migration Conventions

- One concern per migration file (accounts, zones, characters, ...)
- Numbered sequentially: `NNN_description.sql`
- Always include comments comparing to EQEmu's equivalent for traceability
- Prefer JSONB over wide column sprawl for variable-schema data
- Always add indexes for foreign keys and frequent lookup columns

## Protocol Conventions

- One .proto file per game domain (zone, entity, combat, chat, ...)
- All messages imported and wired through `packet.proto`'s `Packet.oneof`
- Field number ranges grouped by category with gaps for future additions
- Enums use `SCREAMING_SNAKE_CASE` with a `_UNSPECIFIED = 0` sentinel
- `buf lint` must pass before committing proto changes
- `buf breaking` against the previous commit to catch wire-breaking changes

## Server Process Boundaries

Zone servers are independent. They must not call other zone servers directly.
All cross-zone communication flows through the world server. A zone server
communicates only with: its own database connection, the world server, and
its connected clients.
