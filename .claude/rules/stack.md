# Standard Stack

## Protocol (Networking)
Protobuf 3 with `buf` for linting and code generation. All message definitions
live in `proto/adif/`. C# namespace: `Adif.Protocol`.

Use the `Packet` envelope with `oneof payload` for all client-server messages.
Field number ranges are allocated by category with gaps. Never reuse a retired
field number — mark it `reserved` instead.

WHY: Protobuf's oneof replaces EQ's brittle opcode dispatch table with
type-safe, schema-enforced routing. buf's breaking-change detection (FILE
policy) catches wire-incompatible changes before they ship.

## Server
C# / .NET. Each server process (Login, World, Zone, Chat) is a separate
executable in the same solution. Shared code goes in a `Common` library project.

WHY: The cluster model requires process isolation. A zone crash must not take
down the world server. Shared code via a library project, not copy-paste.

## Database
PostgreSQL (via Docker, `database/docker-compose.yml`). Schema managed by
numbered SQL migrations in `database/migrations/`. No ORM for game data —
use raw SQL or a thin query builder (Dapper or similar).

Redis for ephemeral state: session tokens, zone population counts, cross-server
pub/sub.

WHY: MMO data is relational (characters have inventory, NPCs reference loot
tables). An ORM's object-graph assumptions fight the ECS-like flat data model.
Raw SQL gives full control over query plans for hot paths.

## JSONB Convention
Use JSONB columns for fields that vary by entity type or have many optional
sub-fields (appearance, resistances, stats, spell effects). Keep indexed
lookup columns as native types (INTEGER, VARCHAR, BOOLEAN).

WHY: EQEmu's items table has 170+ columns, most NULL for any given row. JSONB
collapses these into a single column while keeping PostgreSQL's query engine.

## Client (TBD)
Engine not yet chosen. When chosen, document it here. The server must never
assume a specific client — all rendering data stays client-side. The server
sends entity state via protobuf; the client decides how to draw it.

## Dev Environment
- Docker Compose for infrastructure (PostgreSQL, Redis)
- PowerShell scripts in `scripts/` for setup automation
- `buf lint` and `buf breaking` in CI for protocol safety
- .NET test projects in `tests/` for protocol and logic verification
