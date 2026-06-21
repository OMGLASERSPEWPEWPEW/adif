-- 020_logging_and_analytics.sql
-- Server logging, bug reports, petitions, player event tracking.
--
-- EQEmu: bugs, petitions, reports, discovered_items, saylink, titles,
-- player_titlesets, discord_webhooks, server_scheduled_events,
-- player_event_* tables.
-- ADIF: Identical — operational infrastructure.

-- Player-submitted bug reports
CREATE TABLE IF NOT EXISTS bugs (
    id     SERIAL PRIMARY KEY,
    zone   TEXT NOT NULL DEFAULT '',
    name   TEXT NOT NULL DEFAULT '',
    ui     TEXT NOT NULL DEFAULT '',
    x      REAL NOT NULL DEFAULT 0,
    y      REAL NOT NULL DEFAULT 0,
    z      REAL NOT NULL DEFAULT 0,
    type   TEXT NOT NULL DEFAULT '',
    flag   SMALLINT NOT NULL DEFAULT 0,
    target TEXT NOT NULL DEFAULT '',
    bug    TEXT NOT NULL DEFAULT '',
    date   TIMESTAMP NOT NULL DEFAULT NOW(),
    status SMALLINT NOT NULL DEFAULT 0
);

-- Player petitions (help requests to GMs)
CREATE TABLE IF NOT EXISTS petitions (
    dession    INTEGER PRIMARY KEY,
    petid      INTEGER NOT NULL DEFAULT 0,
    charname   TEXT NOT NULL DEFAULT '',
    accountname TEXT NOT NULL DEFAULT '',
    lastgm     TEXT NOT NULL DEFAULT '',
    petitiontext TEXT NOT NULL DEFAULT '',
    gmtext     TEXT NOT NULL DEFAULT '',
    zone       TEXT NOT NULL DEFAULT '',
    urgency    SMALLINT NOT NULL DEFAULT 0,
    charclass  SMALLINT NOT NULL DEFAULT 0,
    charrace   SMALLINT NOT NULL DEFAULT 0,
    charlevel  SMALLINT NOT NULL DEFAULT 0,
    checkouts  SMALLINT NOT NULL DEFAULT 0,
    unavail    SMALLINT NOT NULL DEFAULT 0,
    senttime   TIMESTAMP NOT NULL DEFAULT NOW()
);

-- GM reports
CREATE TABLE IF NOT EXISTS reports (
    id         SERIAL PRIMARY KEY,
    name       TEXT NOT NULL DEFAULT '',
    reported   TEXT NOT NULL DEFAULT '',
    reported_text TEXT NOT NULL DEFAULT ''
);

-- Items discovered by players (first-find tracking)
CREATE TABLE IF NOT EXISTS discovered_items (
    item_id       INTEGER PRIMARY KEY,
    char_name     TEXT NOT NULL DEFAULT '',
    discovered_date INTEGER NOT NULL DEFAULT 0,
    account_status INTEGER NOT NULL DEFAULT 0
);

-- Clickable text links in chat
CREATE TABLE IF NOT EXISTS saylink (
    id     SERIAL PRIMARY KEY,
    phrase TEXT NOT NULL DEFAULT ''
);

-- Player title definitions
CREATE TABLE IF NOT EXISTS titles (
    id              SERIAL PRIMARY KEY,
    skill_id        SMALLINT NOT NULL DEFAULT -1,
    min_skill_value INTEGER NOT NULL DEFAULT -1,
    max_skill_value INTEGER NOT NULL DEFAULT -1,
    min_aa_points   INTEGER NOT NULL DEFAULT -1,
    max_aa_points   INTEGER NOT NULL DEFAULT -1,
    "class"         SMALLINT NOT NULL DEFAULT -1,
    gender          SMALLINT NOT NULL DEFAULT -1,
    char_id         INTEGER NOT NULL DEFAULT -1,
    status          INTEGER NOT NULL DEFAULT -1,
    item_id         INTEGER NOT NULL DEFAULT -1,
    prefix          TEXT NOT NULL DEFAULT '',
    suffix          TEXT NOT NULL DEFAULT '',
    title_set       INTEGER NOT NULL DEFAULT 0
);

-- Player title set membership
CREATE TABLE IF NOT EXISTS player_titlesets (
    id       SERIAL PRIMARY KEY,
    char_id  INTEGER NOT NULL DEFAULT 0,
    title_set INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_player_titlesets_char ON player_titlesets(char_id);

-- Discord webhook configuration
CREATE TABLE IF NOT EXISTS discord_webhooks (
    id           SERIAL PRIMARY KEY,
    webhook_name TEXT NOT NULL DEFAULT '',
    webhook_url  TEXT NOT NULL DEFAULT '',
    created_at   TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at   TIMESTAMP
);

-- Scheduled server events (timed content)
CREATE TABLE IF NOT EXISTS server_scheduled_events (
    id              SERIAL PRIMARY KEY,
    description     TEXT NOT NULL DEFAULT '',
    event_type      TEXT NOT NULL DEFAULT '',
    event_data      TEXT NOT NULL DEFAULT '',
    minute_start    INTEGER NOT NULL DEFAULT 0,
    hour_start      INTEGER NOT NULL DEFAULT 0,
    day_start       INTEGER NOT NULL DEFAULT 0,
    month_start     INTEGER NOT NULL DEFAULT 0,
    year_start      INTEGER NOT NULL DEFAULT 0,
    minute_end      INTEGER NOT NULL DEFAULT 0,
    hour_end        INTEGER NOT NULL DEFAULT 0,
    day_end         INTEGER NOT NULL DEFAULT 0,
    month_end       INTEGER NOT NULL DEFAULT 0,
    year_end        INTEGER NOT NULL DEFAULT 0,
    cron_expression TEXT NOT NULL DEFAULT '',
    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMP
);

-- Player event logging configuration
CREATE TABLE IF NOT EXISTS player_event_log_settings (
    id                  SERIAL PRIMARY KEY,
    event_name          TEXT NOT NULL DEFAULT '',
    event_enabled       SMALLINT NOT NULL DEFAULT 0,
    retention_days      INTEGER NOT NULL DEFAULT 0,
    discord_webhook_id  INTEGER NOT NULL DEFAULT 0
);

-- Player event log entries
CREATE TABLE IF NOT EXISTS player_event_logs (
    id              BIGSERIAL PRIMARY KEY,
    account_id      INTEGER NOT NULL DEFAULT 0,
    character_id    INTEGER NOT NULL DEFAULT 0,
    zone_id         INTEGER NOT NULL DEFAULT 0,
    instance_id     INTEGER NOT NULL DEFAULT 0,
    x               REAL NOT NULL DEFAULT 0,
    y               REAL NOT NULL DEFAULT 0,
    z               REAL NOT NULL DEFAULT 0,
    heading         REAL NOT NULL DEFAULT 0,
    event_type_id   INTEGER NOT NULL DEFAULT 0,
    event_type_name TEXT NOT NULL DEFAULT '',
    event_data      TEXT NOT NULL DEFAULT '',
    created_at      TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_player_event_logs_char ON player_event_logs(character_id);
CREATE INDEX IF NOT EXISTS idx_player_event_logs_type ON player_event_logs(event_type_id);

-- Item tick effects (periodic item procs)
CREATE TABLE IF NOT EXISTS item_tick (
    it_itemid  INTEGER NOT NULL DEFAULT 0,
    it_chance  INTEGER NOT NULL DEFAULT 0,
    it_level   INTEGER NOT NULL DEFAULT 0,
    it_id      SERIAL PRIMARY KEY,
    it_qglobal TEXT NOT NULL DEFAULT '',
    it_bagslot SMALLINT NOT NULL DEFAULT 0
);

-- Web API character data cache
CREATE TABLE IF NOT EXISTS webdata_character (
    id      INTEGER PRIMARY KEY,
    name    TEXT NOT NULL DEFAULT '',
    last_login TIMESTAMP,
    last_seen  TIMESTAMP
);

-- Web API server status cache
CREATE TABLE IF NOT EXISTS webdata_servers (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL DEFAULT '',
    last_update TIMESTAMP
);

-- Temporary merchant overrides (runtime)
CREATE TABLE IF NOT EXISTS merchantlist_temp (
    npcid    INTEGER NOT NULL DEFAULT 0,
    slot     INTEGER NOT NULL DEFAULT 0,
    itemid   INTEGER NOT NULL DEFAULT 0,
    charges  INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (npcid, slot)
);
