# Anti-Patterns

## NEVER trust the client for game state
The server is authoritative for ALL game state: position validation, damage
calculation, inventory changes, currency, experience. The client sends
*intentions* (I want to attack, I want to move here). The server decides
the outcome.

WHAT HAPPENS: Client-authoritative state enables speed hacks, teleport hacks,
damage hacks, item duplication. Every cheat in MMO history exploits a case
where the server trusted the client.

## NEVER couple zone logic to geometry format
Zone game logic (spawns, combat, quests, loot) must work identically regardless
of whether the zone uses mesh, voxel, or procedural geometry. Access geometry
only through the Map/WaterMap/IPathfinder interfaces.

WHAT HAPPENS: If combat code checks "is this a voxel zone?" to decide behavior,
adding a new geometry backend means auditing every game system. The whole point
of ADIF's pluggable zone architecture is lost.

## NEVER let zone servers communicate directly
Zone-to-zone communication goes through the world server. Zone A must never
open a connection to Zone B, call Zone B's methods, or share memory with Zone B.

WHAT HAPPENS: Direct zone coupling means a crash in one zone can cascade.
It also prevents scaling zones across multiple machines. The world server is
the message bus — that's its job.

## NEVER add columns to PostgreSQL tables when JSONB is appropriate
If a field is optional for most rows, varies by subtype, or has many peer
fields that always appear together, put it in an existing JSONB column. Do
not add 20 nullable columns to match EQEmu's schema.

WHAT HAPPENS: EQEmu's items table has 170+ columns. Most are NULL for any
given item. This makes the schema unreadable and migrations painful. ADIF's
`stats JSONB` and `effects JSONB` pattern is deliberate.

## NEVER reuse or renumber protobuf field numbers
When removing a field from a .proto message, mark it `reserved`. Never assign
its number to a new field. For the Packet oneof, never reuse a payload field
number.

WHAT HAPPENS: Old clients with cached .proto definitions will deserialize the
wrong type into the new field. Silent data corruption. buf's breaking-change
detection catches this, but only if you run it.

## NEVER store ephemeral state in PostgreSQL
Session tokens, "who is online" lists, zone population counts, temporary
combat state — these go in Redis or in-memory. PostgreSQL is for state that
must survive a server restart.

WHAT HAPPENS: Writing every position update or combat tick to PostgreSQL
generates catastrophic write load. Character saves should happen on zone
exit, periodic timer, or explicit /save — not every tick.

## NEVER skip the EQEmu comparison comment in migrations
Every migration file should comment on how ADIF's schema differs from EQEmu's
equivalent table and WHY (e.g., "EQEmu: zone table (92 columns). ADIF:
modernized with JSONB for fog/weather.").

WHAT HAPPENS: Without this, future contributors (or Claude) won't know
whether a missing column is intentional simplification or an oversight. The
comments are the design rationale.
