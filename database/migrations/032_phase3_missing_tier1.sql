-- 032_phase3_missing_tier1.sql
-- Phase 3: Create ~17 missing Tier 1 tables needed for login, character creation,
-- and zone entry.
--
-- These tables are referenced by EQEmu C++ code and cause errors when missing.
-- Schemas extracted from reference/eqemu-server/common/repositories/base/*.h.

BEGIN;

-- ============================================================================
-- AA System (4 tables)
-- ============================================================================
-- EQEmu's modern AA system. Replaces the legacy altadv_vars from migration 017.
-- Loaded by shared_memory at boot. Required for character select and gameplay.

-- aa_ability: AA ability definitions
CREATE TABLE IF NOT EXISTS aa_ability (
    id                  INTEGER NOT NULL PRIMARY KEY,
    name                TEXT NOT NULL DEFAULT '',
    category            INTEGER NOT NULL DEFAULT 0,
    classes             INTEGER NOT NULL DEFAULT 0,
    races               INTEGER NOT NULL DEFAULT 0,
    drakkin_heritage    INTEGER NOT NULL DEFAULT 0,
    deities             INTEGER NOT NULL DEFAULT 0,
    status              INTEGER NOT NULL DEFAULT 0,
    type                INTEGER NOT NULL DEFAULT 0,
    charges             INTEGER NOT NULL DEFAULT 0,
    grant_only          SMALLINT NOT NULL DEFAULT 0,
    first_rank_id       INTEGER NOT NULL DEFAULT 0,
    enabled             SMALLINT NOT NULL DEFAULT 1,
    reset_on_death      SMALLINT NOT NULL DEFAULT 0,
    auto_grant_enabled  SMALLINT NOT NULL DEFAULT 0
);

-- aa_ranks: Per-rank details for each AA ability
CREATE TABLE IF NOT EXISTS aa_ranks (
    id                  INTEGER NOT NULL PRIMARY KEY,
    upper_hotkey_sid    INTEGER NOT NULL DEFAULT -1,
    lower_hotkey_sid    INTEGER NOT NULL DEFAULT -1,
    title_sid           INTEGER NOT NULL DEFAULT -1,
    desc_sid            INTEGER NOT NULL DEFAULT -1,
    cost                INTEGER NOT NULL DEFAULT 1,
    level_req           INTEGER NOT NULL DEFAULT 51,
    spell               INTEGER NOT NULL DEFAULT -1,
    spell_type          INTEGER NOT NULL DEFAULT 0,
    recast_time         INTEGER NOT NULL DEFAULT 0,
    expansion           INTEGER NOT NULL DEFAULT 0,
    prev_id             INTEGER NOT NULL DEFAULT -1,
    next_id             INTEGER NOT NULL DEFAULT -1
);

-- aa_rank_effects: Spell-like effects for each AA rank
CREATE TABLE IF NOT EXISTS aa_rank_effects (
    rank_id             INTEGER NOT NULL DEFAULT 0,
    slot                INTEGER NOT NULL DEFAULT 0,
    effect_id           INTEGER NOT NULL DEFAULT 0,
    base1               INTEGER NOT NULL DEFAULT 0,
    base2               INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (rank_id, slot)
);

-- aa_rank_prereqs: Prerequisites for purchasing an AA rank
CREATE TABLE IF NOT EXISTS aa_rank_prereqs (
    rank_id             INTEGER NOT NULL DEFAULT 0,
    aa_id               INTEGER NOT NULL DEFAULT 0,
    points              INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (rank_id, aa_id)
);

-- ============================================================================
-- alternate_currency: Definition table for alternate currency types
-- ============================================================================
-- Maps currency IDs to item IDs. Loaded at boot.

CREATE TABLE IF NOT EXISTS alternate_currency (
    id                  INTEGER NOT NULL PRIMARY KEY,
    item_id             INTEGER NOT NULL DEFAULT 0
);

-- ============================================================================
-- base_data: Base HP/mana/endurance per class and level
-- ============================================================================
-- Core stats table used by character creation and level-up calculations.
-- Composite PK on (level, class).

CREATE TABLE IF NOT EXISTS base_data (
    level               SMALLINT NOT NULL DEFAULT 0,
    "class"             SMALLINT NOT NULL DEFAULT 0,
    hp                  DOUBLE PRECISION NOT NULL DEFAULT 0,
    mana                DOUBLE PRECISION NOT NULL DEFAULT 0,
    "end"               DOUBLE PRECISION NOT NULL DEFAULT 0,
    hp_regen            DOUBLE PRECISION NOT NULL DEFAULT 0,
    end_regen           DOUBLE PRECISION NOT NULL DEFAULT 0,
    hp_fac              DOUBLE PRECISION NOT NULL DEFAULT 0,
    mana_fac            DOUBLE PRECISION NOT NULL DEFAULT 0,
    end_fac             DOUBLE PRECISION NOT NULL DEFAULT 0,
    PRIMARY KEY (level, "class")
);

-- ============================================================================
-- Faction System (3 tables)
-- ============================================================================
-- faction_base_data was created in migration 030 (renamed from factions).
-- These are the remaining faction tables the C++ code expects.

-- faction_list: Master list of all factions
CREATE TABLE IF NOT EXISTS faction_list (
    id                  INTEGER NOT NULL PRIMARY KEY,
    name                TEXT NOT NULL DEFAULT '',
    base                SMALLINT NOT NULL DEFAULT 0
);

-- faction_list_mod: Faction modifiers (race/class/deity adjustments)
CREATE TABLE IF NOT EXISTS faction_list_mod (
    id                  SERIAL PRIMARY KEY,
    faction_id          INTEGER NOT NULL DEFAULT 0,
    mod                 SMALLINT NOT NULL DEFAULT 0,
    mod_name            TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_faction_list_mod_faction ON faction_list_mod (faction_id);

-- npc_faction_entries: Which factions an NPC faction affects
CREATE TABLE IF NOT EXISTS npc_faction_entries (
    npc_faction_id      INTEGER NOT NULL DEFAULT 0,
    faction_id          INTEGER NOT NULL DEFAULT 0,
    value               INTEGER NOT NULL DEFAULT 0,
    npc_value           SMALLINT NOT NULL DEFAULT 0,
    temp                SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (npc_faction_id, faction_id)
);

-- ============================================================================
-- instance_list_player: Maps characters to zone instances
-- ============================================================================
-- Used by the instancing system for zone entry checks.

CREATE TABLE IF NOT EXISTS instance_list_player (
    id                  INTEGER NOT NULL DEFAULT 0,
    charid              INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (id, charid)
);

-- ============================================================================
-- data_buckets: Generic key-value storage for quests and game state
-- ============================================================================
-- Used extensively by the quest system for persistent state. Supports per-account,
-- per-character, per-NPC, per-zone scoping via optional ID columns.

CREATE TABLE IF NOT EXISTS data_buckets (
    id                  BIGSERIAL PRIMARY KEY,
    "key"               TEXT NOT NULL DEFAULT '',
    value               TEXT NOT NULL DEFAULT '',
    expires             INTEGER NOT NULL DEFAULT 0,
    account_id          BIGINT NOT NULL DEFAULT 0,
    character_id        BIGINT NOT NULL DEFAULT 0,
    npc_id              INTEGER NOT NULL DEFAULT 0,
    bot_id              INTEGER NOT NULL DEFAULT 0,
    zone_id             SMALLINT NOT NULL DEFAULT 0,
    instance_id         SMALLINT NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_data_buckets_key ON data_buckets ("key");
CREATE INDEX IF NOT EXISTS idx_data_buckets_character ON data_buckets (character_id);
CREATE INDEX IF NOT EXISTS idx_data_buckets_npc ON data_buckets (npc_id);
CREATE INDEX IF NOT EXISTS idx_data_buckets_zone ON data_buckets (zone_id);

-- ============================================================================
-- character_stats_record: Snapshot of character stats at various points
-- ============================================================================
-- Used for stat tracking, but not critical for basic gameplay.
-- Create as empty placeholder so the C++ code doesn't error on missing table.

CREATE TABLE IF NOT EXISTS character_stats_record (
    character_id        INTEGER NOT NULL DEFAULT 0,
    name                TEXT NOT NULL DEFAULT '',
    status              INTEGER NOT NULL DEFAULT 0,
    level               INTEGER NOT NULL DEFAULT 0,
    "class"             INTEGER NOT NULL DEFAULT 0,
    race                INTEGER NOT NULL DEFAULT 0,
    aa_points           INTEGER NOT NULL DEFAULT 0,
    hp                  INTEGER NOT NULL DEFAULT 0,
    mana                INTEGER NOT NULL DEFAULT 0,
    endurance           INTEGER NOT NULL DEFAULT 0,
    ac                  INTEGER NOT NULL DEFAULT 0,
    strength            INTEGER NOT NULL DEFAULT 0,
    stamina             INTEGER NOT NULL DEFAULT 0,
    dexterity           INTEGER NOT NULL DEFAULT 0,
    agility             INTEGER NOT NULL DEFAULT 0,
    intelligence        INTEGER NOT NULL DEFAULT 0,
    wisdom              INTEGER NOT NULL DEFAULT 0,
    charisma            INTEGER NOT NULL DEFAULT 0,
    magic_resist        INTEGER NOT NULL DEFAULT 0,
    fire_resist         INTEGER NOT NULL DEFAULT 0,
    cold_resist         INTEGER NOT NULL DEFAULT 0,
    poison_resist       INTEGER NOT NULL DEFAULT 0,
    disease_resist      INTEGER NOT NULL DEFAULT 0,
    corruption_resist   INTEGER NOT NULL DEFAULT 0,
    heroic_strength     INTEGER NOT NULL DEFAULT 0,
    heroic_stamina      INTEGER NOT NULL DEFAULT 0,
    heroic_dexterity    INTEGER NOT NULL DEFAULT 0,
    heroic_agility      INTEGER NOT NULL DEFAULT 0,
    heroic_intelligence INTEGER NOT NULL DEFAULT 0,
    heroic_wisdom       INTEGER NOT NULL DEFAULT 0,
    heroic_charisma     INTEGER NOT NULL DEFAULT 0,
    heroic_magic_resist INTEGER NOT NULL DEFAULT 0,
    heroic_fire_resist  INTEGER NOT NULL DEFAULT 0,
    heroic_cold_resist  INTEGER NOT NULL DEFAULT 0,
    heroic_poison_resist INTEGER NOT NULL DEFAULT 0,
    heroic_disease_resist INTEGER NOT NULL DEFAULT 0,
    heroic_corruption_resist INTEGER NOT NULL DEFAULT 0,
    haste               INTEGER NOT NULL DEFAULT 0,
    accuracy            INTEGER NOT NULL DEFAULT 0,
    attack              INTEGER NOT NULL DEFAULT 0,
    avoidance           INTEGER NOT NULL DEFAULT 0,
    clairvoyance        INTEGER NOT NULL DEFAULT 0,
    combat_effects      INTEGER NOT NULL DEFAULT 0,
    damage_shield       INTEGER NOT NULL DEFAULT 0,
    damage_shield_mitigation INTEGER NOT NULL DEFAULT 0,
    dot_shielding       INTEGER NOT NULL DEFAULT 0,
    hp_regen            INTEGER NOT NULL DEFAULT 0,
    mana_regen          INTEGER NOT NULL DEFAULT 0,
    endurance_regen     INTEGER NOT NULL DEFAULT 0,
    shielding           INTEGER NOT NULL DEFAULT 0,
    spell_damage        INTEGER NOT NULL DEFAULT 0,
    spell_shielding     INTEGER NOT NULL DEFAULT 0,
    strikethrough       INTEGER NOT NULL DEFAULT 0,
    stun_resist         INTEGER NOT NULL DEFAULT 0,
    backstab            INTEGER NOT NULL DEFAULT 0,
    wind                INTEGER NOT NULL DEFAULT 0,
    brass               INTEGER NOT NULL DEFAULT 0,
    string              INTEGER NOT NULL DEFAULT 0,
    percussion          INTEGER NOT NULL DEFAULT 0,
    singing             INTEGER NOT NULL DEFAULT 0,
    baking              INTEGER NOT NULL DEFAULT 0,
    alchemy             INTEGER NOT NULL DEFAULT 0,
    tailoring           INTEGER NOT NULL DEFAULT 0,
    blacksmithing       INTEGER NOT NULL DEFAULT 0,
    fletching           INTEGER NOT NULL DEFAULT 0,
    brewing             INTEGER NOT NULL DEFAULT 0,
    jewelry             INTEGER NOT NULL DEFAULT 0,
    pottery             INTEGER NOT NULL DEFAULT 0,
    research            INTEGER NOT NULL DEFAULT 0,
    poison_making       INTEGER NOT NULL DEFAULT 0,
    tinkering           INTEGER NOT NULL DEFAULT 0,
    created_at          INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_char_stats_record_char ON character_stats_record (character_id);

-- ============================================================================
-- perl_event_export_settings: Quest event configuration
-- ============================================================================
-- Controls which events fire Perl/Lua quest scripts. Loaded at boot.

CREATE TABLE IF NOT EXISTS perl_event_export_settings (
    event_id            INTEGER NOT NULL PRIMARY KEY,
    event_description   TEXT NOT NULL DEFAULT '',
    export_qglobals     SMALLINT NOT NULL DEFAULT 0,
    export_mob          SMALLINT NOT NULL DEFAULT 0,
    export_zone         SMALLINT NOT NULL DEFAULT 0,
    export_item         SMALLINT NOT NULL DEFAULT 0,
    export_event        SMALLINT NOT NULL DEFAULT 0
);

-- ============================================================================
-- instance_list: Active zone instances
-- ============================================================================

CREATE TABLE IF NOT EXISTS instance_list (
    id                  SERIAL PRIMARY KEY,
    zone                INTEGER NOT NULL DEFAULT 0,
    version             SMALLINT NOT NULL DEFAULT 0,
    is_global           SMALLINT NOT NULL DEFAULT 0,
    start_time          INTEGER NOT NULL DEFAULT 0,
    duration            INTEGER NOT NULL DEFAULT 0,
    never_expires       SMALLINT NOT NULL DEFAULT 0
);

COMMIT;
