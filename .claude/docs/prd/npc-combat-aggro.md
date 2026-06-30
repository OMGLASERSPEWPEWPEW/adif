# PRD: NPC-to-Player Combat & Aggro System

**Date:** 2026-06-30
**Status:** Approved for implementation
**Scope:** Protocol bridge (server/adif-bridge)

## Executive Summary

NPCs in the ADIF bridge are passive — they stand still, take damage, die, and drop loot, but never fight back. This feature adds the NPC side of combat: proximity-based aggro detection, faction checks, NPC pursuit/chase, melee attacks against the player, player HP tracking, and player death. The goal is EQEmu parity for the core combat loop.

## Problem

The bridge handles player-to-NPC combat (targeting, auto-attack, damage, death, looting) but NPCs are inert. A player can stand next to a hostile NPC indefinitely without being attacked. This breaks the fundamental EQ gameplay contract.

## Solution

Implement a server-side aggro scan + NPC combat tick that mirrors EQEmu's architecture:

1. **Aggro scan** on a timer detects players within aggro range
2. **Faction check** determines hostility (SCOWLS/THREATENING = aggro)
3. **NPC pursuit** moves NPCs toward their target (position updates sent to client)
4. **NPC melee attack** on a per-NPC timer deals damage to the player
5. **Player HP tracking** with OP_HPUpdate sent to the client
6. **Player death** when HP reaches 0

## Architecture (from EQEmu reference)

### Aggro Detection Flow

```
Every 1s (moving) / 6s (idle):
  For each NPC within 600 units:
    CheckWillAggro():
      1. Distance < aggroradius (default 70)
      2. Not invisible (check see_invis)
      3. Faction is SCOWLS (always) or THREATENING (25% chance)
      4. Not gray con (unless undead/AlwaysAggro)
      5. Line of sight (skip for bridge — no geometry)
    If aggro:
      AddToHateList(player, 25)
```

### NPC Combat Tick

```
Every 250ms (existing combat_tick):
  For each NPC with hate_list entries:
    target = highest hate entry
    if distance(npc, target) > melee_range:
      RunTo(target) → send OP_ClientUpdate (position)
    else:
      if attack_timer expired:
        Calculate damage (min_dmg..max_dmg range)
        Send: OP_Animation (NPC swing)
        Send: OP_Damage (source=NPC, target=player)
        Send: OP_HPUpdate (player's new HP)
        if player HP <= 0:
          Send: OP_Death (player dies)
```

### Key Data Needed

**From npc_types (add to ZoneSpawnRow query):**
- `npc_faction_id` — links to faction tables
- `aggroradius` — detection range (default 70 if 0)

**New fields on SpawnedNpcInfo:**
- `x, y, z` — NPC position (for distance checks + movement)
- `heading` — facing direction
- `npc_faction_id` — faction ID
- `aggro_radius` — detection range
- `hate_target: Option<u32>` — current target spawn_id
- `last_npc_attack_time: Option<Instant>` — per-NPC melee timer

**New fields on ClientState:**
- `cur_hp: i64` — player's current HP
- `max_hp: i64` — player's max HP (from PlayerProfile or level-based)

### Packet Summary

| Packet | Direction | When | Size | Notes |
|--------|-----------|------|------|-------|
| OP_Animation (0x2ACF) | S→C | NPC swings | 4 | Already implemented for player attacks, reuse |
| OP_Damage (0x5C78) | S→C | NPC hits player | 23 | Same struct, swap source/target |
| OP_HPUpdate (0x3BCF) | S→C | Player takes damage | 10 | cur_hp + max_hp + spawn_id |
| OP_ClientUpdate (0x14CB) | S→C | NPC moves | ~36 | Position update for chasing |
| OP_Death (0x6160) | S→C | Player dies | 32 | Already implemented, reuse |

### Faction Simplified (Phase 1)

Full EQEmu faction is complex (per-race/class/deity modifiers, personal standing, 1000+ faction entries). For Phase 1:

**Approach:** Use `npc_faction_id` to look up `npc_faction.primaryfaction`. Query `faction_list` for the base faction. Check `npc_faction_entries` for the player's race. If `value < -500` → SCOWLS → aggro. If no entry → use base faction mod.

**Simplified rule for MVP:** If `npc_faction_id > 0`, look up the faction. Most Innothule NPCs (frogloks, undead) have factions that scowl at trolls. For the initial implementation, any NPC with `npc_faction_id` that resolves to negative value = hostile.

**Future:** Full faction with personal standing, deity mods, faction hits from kills.

### NPC Movement (Chase)

EQEmu NPCs use navmesh pathfinding. Our bridge has no geometry data. For Phase 1:

**Approach:** Direct line movement toward player. Calculate direction vector from NPC to player, move at `runspeed` units per tick. Send `OP_ClientUpdate` with new position. No pathfinding, no collision.

**NPC speed:** `runspeed` field from DB (typically 1.25 for normal mobs). Convert to units/tick: `runspeed * 20 * (tick_interval / 1000)`.

### Damage Calculation (Simplified)

EQEmu has a complex damage pipeline (avoidance, hit chance, AC mitigation, damage tables, crits). For Phase 1:

**Approach:** Random damage between `min_dmg` and `max_dmg` from the DB. No avoidance, no AC. Just raw damage.

**Future:** Hit/miss rolls, AC mitigation, avoidance (riposte/parry/dodge/block).

### Player HP

Player max HP needs to come from somewhere. Options:
- Calculate from level/class/STA (complex, EQEmu formula)
- Use a flat formula: `max_hp = level * 20 + 100` (MVP)
- Load from character_data if stored

**Approach for MVP:** `max_hp = level * 20 + 100`. Level 2 troll SK = 140 HP. Send OP_HPUpdate after every hit.

### Player Death

When player HP <= 0:
1. Send `OP_Death` (same struct, spawn_id = player, killer_id = NPC)
2. Clear all NPC hate lists referencing the player
3. Stop processing combat for this client
4. Player must /camp or reconnect (no respawn system yet)

## Implementation Phases

### Phase 1: NPC Melee Attack (no aggro, no chase)
- Add `cur_hp`/`max_hp` to ClientState
- When player attacks an NPC, the NPC attacks back on its own timer
- Use `min_dmg`/`max_dmg` from DB
- Send OP_Animation + OP_Damage + OP_HPUpdate
- Player death on HP <= 0

### Phase 2: Aggro Detection
- Add `x, y, z, npc_faction_id, aggro_radius` to SpawnedNpcInfo
- Add aggro scan timer (1s interval)
- Distance check: `sqrt((px-nx)^2 + (py-ny)^2) < aggro_radius`
- Simplified faction: hostile if faction resolves negative
- NPC auto-targets player on aggro

### Phase 3: NPC Chase
- NPC moves toward player when out of melee range
- Send OP_ClientUpdate with new NPC position
- Melee range check: ~15 units (size-dependent in EQEmu)
- Leash: NPC returns home if player moves > 300 units from spawn

### Phase 4: Polish
- Assist calls (nearby friendly NPCs join fight)
- Proper faction lookups from DB tables
- NPC de-aggro when player dies or zones
- Corpse/XP for player death

## Acceptance Criteria

- [ ] NPC attacks player back during combat (OP_Damage with source=NPC)
- [ ] Player HP bar decreases when hit (OP_HPUpdate)
- [ ] Player dies when HP reaches 0 (OP_Death)
- [ ] NPCs aggro player on proximity (within aggro_radius)
- [ ] Hostile faction NPCs aggro, neutral/friendly don't
- [ ] NPC chases player (position updates visible in client)
- [ ] NPC stops chasing beyond leash distance
- [ ] No crashes or disconnects during combat

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Combat tick too slow for NPC attacks | NPCs feel sluggish | NPC attack timer is per-NPC, not per-tick |
| No pathfinding → NPCs walk through walls | Visual glitch | Acceptable for Phase 1, flagged for geometry work |
| Faction tables complex | Slow implementation | Start with simplified hostile/neutral check |
| Player death with no respawn | Player stuck | /camp still works, player can relog |
