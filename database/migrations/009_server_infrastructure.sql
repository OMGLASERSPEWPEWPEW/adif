-- 009_server_infrastructure.sql
-- Critical server infrastructure tables required for boot.
--
-- EQEmu: These tables exist in peq_content and are loaded at world/zone startup.
-- ADIF: Identical schema — these are server plumbing, not game content.
-- No JSONB modernization needed here.

-- Server key-value store, loaded at boot for configuration
-- EQEmu: variables table (varname, value, information, ts). Unchanged.
CREATE TABLE IF NOT EXISTS variables (
    varname     TEXT PRIMARY KEY,
    value       TEXT NOT NULL DEFAULT '',
    information TEXT NOT NULL DEFAULT '',
    ts          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Rule sets (named groups of server rules)
-- EQEmu: rule_sets (ruleset_id, name). Unchanged.
CREATE TABLE IF NOT EXISTS rule_sets (
    ruleset_id  SERIAL PRIMARY KEY,
    name        TEXT NOT NULL DEFAULT ''
);

-- Individual rule values within a rule set
-- EQEmu: rule_values (ruleset_id, rule_name, rule_value, notes). Unchanged.
CREATE TABLE IF NOT EXISTS rule_values (
    ruleset_id  INTEGER NOT NULL REFERENCES rule_sets(ruleset_id),
    rule_name   TEXT NOT NULL,
    rule_value  TEXT NOT NULL DEFAULT '',
    notes       TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (ruleset_id, rule_name)
);

-- GM command access levels
-- EQEmu: command_settings (command, access, aliases). Unchanged.
CREATE TABLE IF NOT EXISTS command_settings (
    command     TEXT PRIMARY KEY,
    access      INTEGER NOT NULL DEFAULT 0,
    aliases     TEXT NOT NULL DEFAULT ''
);

-- Sub-command access levels
-- EQEmu: command_subsettings. Unchanged.
CREATE TABLE IF NOT EXISTS command_subsettings (
    id                  SERIAL PRIMARY KEY,
    parent_command      TEXT NOT NULL DEFAULT '',
    sub_command         TEXT NOT NULL DEFAULT '',
    access_level        INTEGER NOT NULL DEFAULT 0,
    top_level_aliases   TEXT NOT NULL DEFAULT ''
);

-- Logging system category configuration
-- EQEmu: logsys_categories. Controls what gets logged where.
CREATE TABLE IF NOT EXISTS logsys_categories (
    log_category_id          INTEGER PRIMARY KEY,
    log_category_description TEXT NOT NULL DEFAULT '',
    log_to_console           SMALLINT NOT NULL DEFAULT 0,
    log_to_file              SMALLINT NOT NULL DEFAULT 0,
    log_to_gmsay             SMALLINT NOT NULL DEFAULT 0,
    log_to_discord           SMALLINT NOT NULL DEFAULT 0,
    discord_webhook_id       INTEGER NOT NULL DEFAULT 0
);

-- Expansion/content gating flags
-- EQEmu: content_flags. Controls which content is active.
CREATE TABLE IF NOT EXISTS content_flags (
    id       SERIAL PRIMARY KEY,
    flag_name TEXT NOT NULL DEFAULT '',
    enabled  SMALLINT NOT NULL DEFAULT 1,
    notes    TEXT NOT NULL DEFAULT ''
);
CREATE UNIQUE INDEX IF NOT EXISTS idx_content_flags_name ON content_flags(flag_name);

-- Quest/script key-value data store with expiration
-- EQEmu: data_buckets. General-purpose persistent storage for scripts.
CREATE TABLE IF NOT EXISTS data_buckets (
    id      SERIAL PRIMARY KEY,
    "key"   TEXT NOT NULL DEFAULT '',
    value   TEXT NOT NULL DEFAULT '',
    expires INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_data_buckets_key ON data_buckets("key");

-- Character name filter (banned names)
-- EQEmu: name_filter (name). Single-column filter table.
CREATE TABLE IF NOT EXISTS name_filter (
    name TEXT PRIMARY KEY
);

-- Class/level skill cap definitions
-- EQEmu: skill_caps (skillID, class, level, cap). ADIF: same purpose, cleaner names.
CREATE TABLE IF NOT EXISTS skill_caps (
    id        SERIAL PRIMARY KEY,
    skill_id  SMALLINT NOT NULL DEFAULT 0,
    class_id  SMALLINT NOT NULL DEFAULT 0,
    level     SMALLINT NOT NULL DEFAULT 0,
    cap       INTEGER NOT NULL DEFAULT 0,
    "class"   SMALLINT NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_skill_caps_lookup ON skill_caps(skill_id, class_id, level);
