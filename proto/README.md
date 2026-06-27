# ADIF Protocol

Protobuf 3 message definitions for all ADIF client-server and
server-to-server communication. Replaces EQEmu's hand-rolled C structs
and opcode dispatch with type-safe, schema-enforced routing.

## Files

| File | Purpose |
|------|---------|
| `packet.proto` | Client-server envelope — `Packet` with 173-message `oneof payload` |
| `ipc.proto` | Server-to-server envelope — `IpcMessage` with 77-message `oneof payload` |
| `common.proto` | Shared types: `Vec3`, `Vec3Int`, `Color` |
| `connection.proto` | Session, auth, heartbeat, disconnect |
| `zone.proto` | Zone entry, config, transitions, weather, time |
| `entity.proto` | Spawn (43 fields), despawn, position, appearance |
| `character.proto` | Character list/create, PlayerProfile (57 fields) |
| `combat.proto` | Melee, spells, buffs, death, targeting |
| `chat.proto` | Chat messages, emotes, system messages |
| `inventory.proto` | Items (40 fields), loot, inventory, containers, money |
| `trade.proto` | Player trade, merchants, bazaar |
| `group.proto` | Group/raid management |
| `guild.proto` | Guild management |
| `skills.proto` | Class abilities, tradeskills, tracking |
| `social.proto` | Who, inspect, duels, resurrection, pets |
| `admin.proto` | GM commands |

## Adding a New Message

1. Define the message in the appropriate domain `.proto` file
2. Add it to `packet.proto`'s `oneof payload` (client-server) or
   `ipc.proto`'s `oneof payload` (server-to-server)
3. Use the next available field number in the category range (see ranges
   in `packet.proto` comments)
4. Run `.\scripts\proto-check.ps1` to verify lint, build, and breaking
5. Add a round-trip test in `tests/ProtoRoundTrip/Program.cs`

## Field Number Policy

- Never reuse a retired field number — mark it `reserved`
- Client-server ranges in `packet.proto`: Connection 10-19, Zone 20-39,
  Entity 40-59, Character 60-79, Combat 80-99, etc.
- IPC ranges in `ipc.proto`: Tier 1 (lifecycle) 10-39, Tier 2
  (cross-zone) 40-79, Tier 3 (admin) 80-109
- Leave gaps between categories for future additions

## Tooling

- **`buf lint proto/`** — STANDARD rules, `PACKAGE_VERSION_SUFFIX` excepted
- **`buf build proto/`** — compile all proto files
- **`buf breaking proto/ --against proto/image.bin`** — detect wire-breaking changes
- **`dotnet run --project tests/ProtoRoundTrip/`** — 97 round-trip tests
- **`.\scripts\proto-check.ps1`** — runs all four checks in sequence

## Code Generation

C# code is generated at build time via Grpc.Tools MSBuild integration
(see `tests/ProtoRoundTrip/ProtoRoundTrip.csproj`). For other languages,
`buf.gen.yaml` can be extended with additional plugins.
