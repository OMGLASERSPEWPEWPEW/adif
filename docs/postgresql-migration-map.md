# PostgreSQL Migration Map

Single source of truth for the EQEmu MySQL→PostgreSQL table migration.
Generated 2026-06-21. Updated as tables are migrated.

## Summary

| Category | Count |
|----------|-------|
| Tables EQEmu code expects | 250 |
| Tables PostgreSQL has | 129 |
| Tables that match (name exists in both) | 104 |
| Tables missing from PostgreSQL | 146 |
| Tables in PG not referenced by EQEmu repos | 25 |
| Tables with name mismatches | 9 |
| Tables with column mismatches (of matched) | ~6+ |

## Priority Tiers

### Tier 1: CRITICAL — Needed for login + character creation + zone entry
These block basic gameplay. Fix first.

| EQEmu Table | PG Status | Issue | Notes |
|-------------|-----------|-------|-------|
| `character_data` | EXISTS (renamed) | Column mismatch: 106 expected, 51 present | Was `characters`, renamed. Needs full schema from MySQL |
| `npc_types` | RENAME NEEDED | PG has `npc_templates` (131 cols) | Rename or add synonym |
| `zone` | RENAME NEEDED | PG has `zones` (108 cols) | Rename or add synonym |
| `spells_new` | RENAME NEEDED | PG has `spells` (237 cols) | Rename or add synonym |
| `inventory` | RENAME NEEDED | PG has `character_inventory` | Rename or add synonym |
| `faction_values` | RENAME NEEDED | PG has `character_faction_values` | Rename or add synonym |
| `timers` | RENAME NEEDED | PG has `character_timers` | Rename or add synonym |
| `zone_flags` | RENAME NEEDED | PG has `character_zone_flags` | Rename or add synonym |
| `faction_base_data` | MISSING | PG has `factions` (different schema?) | Check if rename or new table |
| `faction_association` | MISSING | PG has `npc_faction_associations` | Check if rename or new table |
| `instance_list_player` | MISSING | Needed for zone instancing | Create from MySQL schema |
| `base_data` | MISSING | Base stats per class/level | Create from MySQL schema |
| `aa_ability` | MISSING | AA system | Create from MySQL schema |
| `aa_ranks` | MISSING | AA system | Create from MySQL schema |
| `aa_rank_effects` | MISSING | AA system | Create from MySQL schema |
| `aa_rank_prereqs` | MISSING | AA system | Create from MySQL schema |
| `alternate_currency` | MISSING | Alt currency (radiant/ebon crystals etc) | Create from MySQL schema |

### Tier 2: IMPORTANT — Needed for gameplay features
These block specific game systems but not basic login/movement.

| EQEmu Table | PG Status | Notes |
|-------------|-----------|-------|
| `character_activities` | MISSING | Task/quest tracking |
| `character_alt_currency` | MISSING | Per-char alt currency |
| `character_auras` | MISSING | Aura effects |
| `character_bandolier` | MISSING | Weapon sets |
| `character_disciplines` | MISSING | Combat abilities |
| `character_evolving_items` | MISSING | Evolving items |
| `character_exp_modifiers` | MISSING | XP modifiers per zone |
| `character_expedition_lockouts` | MISSING | Raid lockouts |
| `character_instance_safereturns` | MISSING | Instance return points |
| `character_item_recast` | MISSING | Item cooldowns |
| `character_leadership_abilities` | MISSING | Leadership AAs |
| `character_parcels` | MISSING | Parcel system |
| `character_parcels_containers` | MISSING | Parcel containers |
| `character_peqzone_flags` | MISSING | PEQ zone flags |
| `character_pet_name` | MISSING | Pet names |
| `character_potionbelt` | MISSING | Potion belt |
| `character_stats_record` | MISSING | Stats history |
| `character_task_timers` | MISSING | Task cooldowns |
| `character_tasks` | MISSING | Active tasks |
| `character_tribute` | MISSING | Tribute system |
| `char_recipe_list` | MISSING | Known recipes |
| `tasks` | MISSING | Task definitions |
| `task_activities` | MISSING | Task steps |
| `tasksets` | MISSING | Task groups |
| `completed_tasks` | MISSING | Completed task history |
| `shared_tasks` | MISSING | Shared tasks |
| `shared_task_members` | MISSING | Shared task members |
| `shared_task_activity_state` | MISSING | Shared task progress |
| `shared_task_dynamic_zones` | MISSING | Shared task instances |
| `completed_shared_tasks` | MISSING | Completed shared tasks |
| `completed_shared_task_members` | MISSING | Completed shared task members |
| `completed_shared_task_activity_state` | MISSING | Completed shared task progress |
| `dynamic_zones` | MISSING | Dynamic zone instances |
| `dynamic_zone_members` | MISSING | DZ membership |
| `dynamic_zone_lockouts` | MISSING | DZ lockouts |
| `dynamic_zone_templates` | MISSING | DZ templates |
| `global_loot` | MISSING | Global loot tables |
| `guild_bank` | MISSING | Guild bank |
| `guild_permissions` | MISSING | Guild permissions |
| `guild_relations` | MISSING | Guild alliances/wars |
| `guild_tributes` | MISSING | Guild tribute |
| `adventure_template` | MISSING | LDoN adventures |
| `adventure_template_entry` | MISSING | LDoN adventure entries |
| `adventure_template_entry_flavor` | MISSING | LDoN flavor text |
| `adventure_details` | MISSING | Active adventures |
| `adventure_members` | MISSING | Adventure members |
| `adventure_stats` | MISSING | Adventure stats |
| `auras` | MISSING | Aura definitions |
| `books` | MISSING | In-game books |
| `buyer` | MISSING | Buyer/barter system |
| `buyer_buy_lines` | MISSING | Buyer buy lines |
| `buyer_trade_items` | MISSING | Buyer trade items |
| `bug_reports` | MISSING | In-game bug reports |
| `chatchannels` | MISSING | Chat channels |
| `chatchannel_reserved_names` | MISSING | Reserved channel names |
| `db_str` | MISSING | Database strings |
| `horses` | MISSING | Horse data |
| `ip_exemptions` | MISSING | IP limit exemptions |
| `items_evolving_details` | MISSING | Evolving item details |
| `keyring` | MISSING | Key ring |
| `ldon_trap_entries` | MISSING | LDoN traps |
| `ldon_trap_templates` | MISSING | LDoN trap templates |
| `lfguild` | MISSING | Looking for guild |
| `mail` | MISSING | In-game mail |
| `npc_scale_global_base` | MISSING | NPC scaling |
| `perl_event_export_settings` | MISSING | Quest event config |
| `pets_beastlord_data` | MISSING | Beastlord pets |
| `pets_equipmentset` | MISSING | Pet equipment |
| `pets_equipmentset_entries` | MISSING | Pet equipment entries |
| `sharedbank` | MISSING | Shared bank |
| `spawn2_disabled` | MISSING | Disabled spawns |
| `spell_buckets` | MISSING | Spell data buckets |
| `tool_game_objects` | MISSING | Dev tool objects |
| `tribute_levels` | MISSING | Tribute level data |
| `tributes` | MISSING | Tribute definitions |
| `veteran_reward_templates` | MISSING | Veteran rewards |
| `zone_state_spawns` | MISSING | Zone state spawns |
| `inventory_snapshots` | MISSING | Inventory snapshots |

### Tier 3: OPTIONAL — Bot/Merc systems (can defer)

| EQEmu Table | Notes |
|-------------|-------|
| `bot_blocked_buffs` | Bot system |
| `bot_buffs` | Bot system |
| `bot_create_combinations` | Bot system |
| `bot_data` | Bot system |
| `bot_group_members` | Bot system |
| `bot_groups` | Bot system |
| `bot_guild_members` | Bot system |
| `bot_heal_rotation_members` | Bot system |
| `bot_heal_rotation_targets` | Bot system |
| `bot_heal_rotations` | Bot system |
| `bot_inspect_messages` | Bot system |
| `bot_inventories` | Bot system |
| `bot_owner_options` | Bot system |
| `bot_pet_buffs` | Bot system |
| `bot_pet_inventories` | Bot system |
| `bot_pets` | Bot system |
| `bot_settings` | Bot system |
| `bot_spell_casting_chances` | Bot system |
| `bot_spell_settings` | Bot system |
| `bot_spells_entries` | Bot system |
| `bot_stances` | Bot system |
| `bot_starting_items` | Bot system |
| `bot_timers` | Bot system |
| `merc_armorinfo` | Mercenary system |
| `merc_buffs` | Mercenary system |
| `merc_inventory` | Mercenary system |
| `merc_merchant_entries` | Mercenary system |
| `merc_merchant_template_entries` | Mercenary system |
| `merc_merchant_templates` | Mercenary system |
| `merc_name_types` | Mercenary system |
| `merc_npc_types` | Mercenary system |
| `merc_spell_list_entries` | Mercenary system |
| `merc_spell_lists` | Mercenary system |
| `merc_stance_entries` | Mercenary system |
| `merc_stats` | Mercenary system |
| `merc_subtypes` | Mercenary system |
| `merc_templates` | Mercenary system |
| `merc_types` | Mercenary system |
| `merc_weaponinfo` | Mercenary system |
| `mercs` | Mercenary system |
| `player_event_aa_purchase` | Player event logging |
| `player_event_killed_named_npc` | Player event logging |
| `player_event_killed_npc` | Player event logging |
| `player_event_killed_raid_npc` | Player event logging |
| `player_event_loot_items` | Player event logging |
| `player_event_merchant_purchase` | Player event logging |
| `player_event_merchant_sell` | Player event logging |
| `player_event_npc_handin` | Player event logging |
| `player_event_npc_handin_entries` | Player event logging |
| `player_event_speech` | Player event logging |
| `player_event_trade` | Player event logging |
| `player_event_trade_entries` | Player event logging |

## Matched Tables with Column Mismatches

These tables exist in both but may have wrong column counts or names.

| Table | EQEmu Cols | PG Cols | Status |
|-------|-----------|---------|--------|
| `account` | 25 | 28 | PG has extra cols (created_at, updated_at, email) — likely OK |
| `character_data` | 106 | 51 | **BROKEN** — needs full rebuild from MySQL schema |
| `items` | 285 | 287 | Close — verify column names match |
| `doors` | 37 | 33 | Missing 4 columns |
| `guilds` | 10 | 5 | Missing 5 columns |
| `guild_members` | 10 | 9 | Missing 1 column |
| `skill_caps` | 6 | 7 | PG has extra col — likely OK |
| `zone_points` | 24 | 24 | Match |
| `spawn2` | 19 | 19 | Match |
| `spawngroup` | 13 | 13 | Match |
| `npc_spells` | 21 | 21 | Match |
| `rule_values` | 4 | 4 | Match |
| `lootdrop` | 6 | 6 | Match |
| `loottable` | 10 | 10 | Match |

## Name Mismatches (PG table exists under different name)

| EQEmu Expects | PostgreSQL Has | Action Needed |
|---------------|----------------|---------------|
| `zone` | `zones` | Rename `zones` → `zone` |
| `npc_types` | `npc_templates` | Rename `npc_templates` → `npc_types` |
| `spells_new` | `spells` | Rename `spells` → `spells_new` |
| `inventory` | `character_inventory` | Rename `character_inventory` → `inventory` |
| `faction_values` | `character_faction_values` | Rename `character_faction_values` → `faction_values` |
| `timers` | `character_timers` | Rename `character_timers` → `timers` |
| `zone_flags` | `character_zone_flags` | Rename `character_zone_flags` → `zone_flags` |
| `faction_base_data` | `factions` | Rename `factions` → `faction_base_data` (verify cols) |
| `faction_association` | `npc_faction_associations` | Rename `npc_faction_associations` → `faction_association` (verify cols) |

## Extra Tables in PostgreSQL (not referenced by EQEmu repos)

These may be ADIF-custom tables or legacy names. Review before removing.

| PG Table | Notes |
|----------|-------|
| `aa_actions` | ADIF custom or legacy EQEmu? |
| `aa_effects` | ADIF custom or legacy EQEmu? |
| `accounts` | Duplicate of `account`? |
| `altadv_vars` | Legacy AA variables |
| `banned_ips` | Not in repos but likely used elsewhere |
| `command_settings` | Legacy command config |
| `db_version` | Schema version tracking |
| `eqtime` | EQ time of day |
| `item_tick` | Item tick data |
| `merchant_entries` | Might be legacy name for merchantlist |
| `tblloginserveraccounts` | Legacy login tables |
| `tblserveradminregistration` | Legacy login tables |
| `tblserverlisttype` | Legacy login tables |
| `tblworldserverregistration` | Legacy login tables |
| `webdata_character` | Web API data |
| `webdata_servers` | Web API data |

## C++ Code Issues (MySQL-isms)

### Base Repositories (`common/repositories/base/*.h`) — 250 files

Converted 2026-06-22 via `scripts/convert-repos-to-pg.py`. 244 files modified,
6 files had no MySQL-isms (bot_group_members, bot_groups, bot_guild_members,
keyring, launcher, tool_game_objects).

| Issue | Count | Status | Notes |
|-------|-------|--------|-------|
| `REPLACE INTO` → `INSERT...ON CONFLICT DO UPDATE` | 244 files | **DONE** | BaseReplace→BaseUpsert, added BaseUpsertSet() |
| `FROM_UNIXTIME()` → `TO_TIMESTAMP()` | 270 instances | **DONE** | In InsertOne/Many, UpdateOne, ReplaceOne/Many |
| `UNIX_TIMESTAMP()` → `EXTRACT(EPOCH FROM)::int` | 54 instances | **DONE** | In SelectColumns arrays (39 files) |
| Backtick → double-quote identifier quoting | 40 instances | **DONE** | Reserved words: class, int, interval, rank, key, range, group |

### Custom Repositories (`common/repositories/*.h`) — ~27 files

Converted 2026-06-22 manually. 15 files modified, 462 backticks stripped from 37 files.

| Issue | Count | Location | Status |
|-------|-------|----------|--------|
| `ON DUPLICATE KEY UPDATE` → `ON CONFLICT DO UPDATE/NOTHING` | 10 | expedition_lockouts, dynamic_zone_*, instance_list_player, char_recipe_list, character_instance_safereturns | **DONE** |
| `FROM_UNIXTIME()` → `TO_TIMESTAMP()` | 4 | expedition_lockouts, dynamic_zone_lockouts | **DONE** |
| `UNIX_TIMESTAMP()` → `EXTRACT(EPOCH FROM)::int` | 8 | character_corpses, character_data, dynamic_zones, instance_list, respawn_times | **DONE** |
| `REPLACE INTO` → `INSERT...ON CONFLICT` | 3 | account_flags, instance_list_player, rule_values | **DONE** |
| `TIMESTAMPDIFF()` → `EXTRACT(EPOCH FROM)::int` | 1 | account_repository | **DONE** |
| `IFNULL()` → `COALESCE()` | 1 | login_accounts_repository | **DONE** |
| `FIELD()` → `array_position()/CASE` | 2 | expedition_lockouts | **DONE** |
| Backtick quoting → stripped | 462 | 37 files | **DONE** |

### Zone Server (`zone/*.cpp`) — ~10 files

Converted 2026-06-22 via parallel agents. Backticks stripped from 27+ zone files.

| Issue | Count | Location | Status |
|-------|-------|----------|--------|
| `REPLACE INTO` → `INSERT...ON CONFLICT` | 12 | mob, questmgr, raids, task_manager, task_client_state, tradeskills, zonedb, bot_database | **DONE** |
| `UPDATE...LIMIT 1` → remove LIMIT | 14 | groups(10), raids(4) | **DONE** |
| `ON DUPLICATE KEY UPDATE` → `ON CONFLICT` | 3 | exp, tradeskills, zonedb | **DONE** |
| `UNIX_TIMESTAMP()`/`MOD()` → `EXTRACT`/`%` | 6 | exp, client, zonedb, mob, questmgr, show_who | **DONE** |
| Backtick quoting → stripped | 500+ | 27+ zone files | **DONE** |

### Common/World/Login — ~8 files

Converted 2026-06-22 via parallel agents. 424 backticks stripped from 11 common files.
World and login server files had no MySQL-isms (already clean).

| Issue | Count | Location | Status |
|-------|-------|----------|--------|
| `ON DUPLICATE KEY UPDATE` → `ON CONFLICT` | 1 | database.cpp (account_ip) | **DONE** |
| `INSERT...SET` → `INSERT INTO...VALUES` | 1 | database.cpp (account_ip) | **DONE** |
| `SHOW TABLE STATUS` → `pg_class` query | 1 | database.cpp | **DONE** |
| `REPLACE INTO` → `INSERT...ON CONFLICT` | 4 | database.cpp, profanity_manager, ptimer, shareddb | **DONE** |
| `IFNULL()` → `COALESCE()` | 1 | database_instances.cpp | **DONE** |
| `UNIX_TIMESTAMP()` → `EXTRACT(EPOCH FROM)::int` | 6 | database.cpp(3), shareddb(2), database_instances(1) | **DONE** |
| Backtick quoting → stripped | 424 | 11 common files | **DONE** |
| `GROUP_CONCAT()` in manifests | 4 | database_update_manifest.h | SKIPPED (legacy MySQL DDL, not runtime) |
| `SHOW TABLES` in manifests | ~70 | database_update_manifest*.h | SKIPPED (legacy MySQL DDL, not runtime) |

### Other

| Issue | Count | Location | Status |
|-------|-------|----------|--------|
| `AUTO_INCREMENT` | Migrations | Table DDL | → `SERIAL` / `GENERATED` |
| `GetVariableInt` for port | 1 | `loginserver/main.cpp:50` | **DONE** (fixed in prior session) |
| Perl generator outputs MySQL | 1 | `utils/scripts/generators/repository-generator.pl` + `template/base_repository.template` | **DONE** — outputs PG-native SQL |
| `RewriteQuery()` runtime layer | 1 | `common/dbcore.cpp` | **DONE** — gutted to passthrough |

## Migration Approach

1. **Phase 1: Name fixes** — Rename 9 mismatched tables
2. **Phase 2: Schema fixes** — Rebuild `character_data`, fix column mismatches on `doors`, `guilds`, `guild_members`
3. **Phase 3: Missing Tier 1 tables** — Create ~17 critical tables from MySQL schemas
4. **Phase 4: Missing Tier 2 tables** — Create ~80 gameplay tables
5. **Phase 5: C++ fixes** — `REPLACE INTO` and other MySQL-isms
6. **Phase 6: Tier 3 (optional)** — Bot/merc tables if needed
7. **Test at each phase** — restart servers, try to get further into the game

## Verification Checkpoints

- [ ] Login to server list
- [ ] Select server, reach character select
- [ ] Create character (name approved, saved to DB)
- [ ] Enter zone (zone boots, client loads)
- [ ] Move around, see NPCs
- [ ] Combat works
- [ ] Zone transition works
- [ ] Character saves on logout
- [ ] Re-login loads saved character
