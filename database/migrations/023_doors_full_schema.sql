-- 023_doors_full_schema.sql
-- Replace simplified ADIF doors table with full EQEmu-compatible schema.
--
-- EQEmu: doors table has 33 columns covering door mechanics, triggers,
-- lockpicking, zone transitions, and client version gating.
-- ADIF 007_factions_and_guilds.sql created a simplified 16-column version.
-- The C++ zone code needs all 33 columns to function.

DROP TABLE IF EXISTS doors CASCADE;

CREATE TABLE IF NOT EXISTS doors (
    id                      SERIAL PRIMARY KEY,
    doorid                  SMALLINT NOT NULL DEFAULT 0,
    zone                    TEXT NOT NULL DEFAULT '',
    name                    TEXT NOT NULL DEFAULT '',
    pos_y                   REAL NOT NULL DEFAULT 0,
    pos_x                   REAL NOT NULL DEFAULT 0,
    pos_z                   REAL NOT NULL DEFAULT 0,
    heading                 REAL NOT NULL DEFAULT 0,
    opentype                SMALLINT NOT NULL DEFAULT 0,
    lockpick                SMALLINT NOT NULL DEFAULT 0,
    keyitem                 INTEGER NOT NULL DEFAULT 0,
    altkeyitem              INTEGER NOT NULL DEFAULT 0,
    nokeyring               SMALLINT NOT NULL DEFAULT 0,
    triggerdoor             SMALLINT NOT NULL DEFAULT 0,
    triggertype             SMALLINT NOT NULL DEFAULT 0,
    doorisopen              SMALLINT NOT NULL DEFAULT 0,
    door_param              INTEGER NOT NULL DEFAULT 0,
    dest_zone               TEXT NOT NULL DEFAULT 'NONE',
    dest_x                  REAL NOT NULL DEFAULT 0,
    dest_y                  REAL NOT NULL DEFAULT 0,
    dest_z                  REAL NOT NULL DEFAULT 0,
    dest_heading            REAL NOT NULL DEFAULT 0,
    invert_state            INTEGER NOT NULL DEFAULT 0,
    incline                 INTEGER NOT NULL DEFAULT 0,
    size                    SMALLINT NOT NULL DEFAULT 100,
    client_version_mask     INTEGER NOT NULL DEFAULT 2147483647,
    islift                  SMALLINT NOT NULL DEFAULT 0,
    close_time              INTEGER NOT NULL DEFAULT 5,
    can_open                SMALLINT NOT NULL DEFAULT 1,
    min_expansion           SMALLINT NOT NULL DEFAULT -1,
    max_expansion           SMALLINT NOT NULL DEFAULT -1,
    content_flags           TEXT NOT NULL DEFAULT '',
    content_flags_disabled  TEXT NOT NULL DEFAULT ''
);

CREATE INDEX IF NOT EXISTS idx_doors_zone ON doors(zone);
CREATE INDEX IF NOT EXISTS idx_doors_doorid ON doors(doorid, zone);
