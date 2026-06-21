-- 017_aa_system.sql
-- Alternate Advancement system definitions.
--
-- EQEmu: aa_actions, aa_effects, altadv_vars. These define the AA abilities
-- available in-game. Character AA purchases are in character_alternate_abilities.
-- ADIF: Identical — AA system is data-driven, not code-driven.

-- AA ability actions (what happens when you activate an AA)
-- EQEmu: aa_actions (12 columns). Unchanged.
CREATE TABLE IF NOT EXISTS aa_actions (
    aaid             INTEGER NOT NULL DEFAULT 0,
    rank_            SMALLINT NOT NULL DEFAULT 0,
    reuse_time       INTEGER NOT NULL DEFAULT 0,
    spell_id         INTEGER NOT NULL DEFAULT 0,
    target           SMALLINT NOT NULL DEFAULT 0,
    nonspell_action  SMALLINT NOT NULL DEFAULT 0,
    nonspell_mana    INTEGER NOT NULL DEFAULT 0,
    nonspell_duration INTEGER NOT NULL DEFAULT 0,
    redux_aa         INTEGER NOT NULL DEFAULT 0,
    redux_rate       INTEGER NOT NULL DEFAULT 0,
    redux_aa2        INTEGER NOT NULL DEFAULT 0,
    redux_rate2      INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (aaid, rank_)
);

-- AA ability passive effects
-- EQEmu: aa_effects (6 columns). Unchanged.
CREATE TABLE IF NOT EXISTS aa_effects (
    id       SERIAL PRIMARY KEY,
    aaid     INTEGER NOT NULL DEFAULT 0,
    slot     SMALLINT NOT NULL DEFAULT 0,
    effectid INTEGER NOT NULL DEFAULT 0,
    base1    INTEGER NOT NULL DEFAULT 0,
    base2    INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_aa_effects_aaid ON aa_effects(aaid);

-- AA ability definitions (name, cost, prerequisites, class restrictions)
-- EQEmu: altadv_vars (24 columns). Large definition table loaded at boot.
CREATE TABLE IF NOT EXISTS altadv_vars (
    skill_id             INTEGER PRIMARY KEY,
    name                 TEXT NOT NULL DEFAULT '',
    cost                 INTEGER NOT NULL DEFAULT 0,
    max_level            SMALLINT NOT NULL DEFAULT 0,
    hotkey_sid           INTEGER NOT NULL DEFAULT 0,
    hotkey_sid2          INTEGER NOT NULL DEFAULT 0,
    title_sid            INTEGER NOT NULL DEFAULT 0,
    desc_sid             INTEGER NOT NULL DEFAULT 0,
    type                 SMALLINT NOT NULL DEFAULT 0,
    spellid              INTEGER NOT NULL DEFAULT 0,
    prereq_skill         INTEGER NOT NULL DEFAULT 0,
    prereq_minpoints     INTEGER NOT NULL DEFAULT 0,
    spell_type           SMALLINT NOT NULL DEFAULT 0,
    spell_refresh        INTEGER NOT NULL DEFAULT 0,
    classes              INTEGER NOT NULL DEFAULT 0,
    berserker            SMALLINT NOT NULL DEFAULT 0,
    class_type           SMALLINT NOT NULL DEFAULT 0,
    cost_inc             INTEGER NOT NULL DEFAULT 0,
    aa_expansion         SMALLINT NOT NULL DEFAULT 0,
    special_category     INTEGER NOT NULL DEFAULT 0,
    account_time_required INTEGER NOT NULL DEFAULT 0,
    level_inc            SMALLINT NOT NULL DEFAULT 0,
    eqmacid              INTEGER NOT NULL DEFAULT 0,
    eqmactype            SMALLINT NOT NULL DEFAULT 0
);
