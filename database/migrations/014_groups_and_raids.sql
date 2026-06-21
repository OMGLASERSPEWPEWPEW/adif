-- 014_groups_and_raids.sql
-- Group and raid persistence, friends list.
--
-- EQEmu: group_id, group_leaders, raid_details, raid_members.
-- ADIF: Identical — these are ephemeral session state persisted for crash recovery.

-- Group membership (persisted for crash recovery)
-- EQEmu: group_id. Unchanged.
CREATE TABLE IF NOT EXISTS group_id (
    charid    INTEGER PRIMARY KEY,
    groupid   INTEGER NOT NULL DEFAULT 0,
    name      TEXT NOT NULL DEFAULT '',
    accountid INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_group_id_groupid ON group_id(groupid);

-- Current group leaders
-- EQEmu: group_leaders. Unchanged.
CREATE TABLE IF NOT EXISTS group_leaders (
    gid        INTEGER PRIMARY KEY,
    leadername TEXT NOT NULL DEFAULT ''
);

-- Raid state (loot rules, lock status)
-- EQEmu: raid_details. Unchanged.
CREATE TABLE IF NOT EXISTS raid_details (
    raidid   INTEGER PRIMARY KEY,
    loottype SMALLINT NOT NULL DEFAULT 0,
    locked   BOOLEAN NOT NULL DEFAULT FALSE
);

-- Raid membership
-- EQEmu: raid_members. Unchanged.
CREATE TABLE IF NOT EXISTS raid_members (
    id            SERIAL PRIMARY KEY,
    raidid        INTEGER NOT NULL DEFAULT 0,
    charid        INTEGER NOT NULL DEFAULT 0,
    groupid       INTEGER NOT NULL DEFAULT 0,
    _class        SMALLINT NOT NULL DEFAULT 0,
    level         SMALLINT NOT NULL DEFAULT 0,
    name          TEXT NOT NULL DEFAULT '',
    isgroupleader BOOLEAN NOT NULL DEFAULT FALSE,
    israidleader  BOOLEAN NOT NULL DEFAULT FALSE,
    islooter      BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX IF NOT EXISTS idx_raid_members_raidid ON raid_members(raidid);

-- Friends list
-- EQEmu: friends. Simple character-to-character friendship.
CREATE TABLE IF NOT EXISTS friends (
    charid   INTEGER NOT NULL DEFAULT 0,
    type     SMALLINT NOT NULL DEFAULT 0,
    name     TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (charid, type, name)
);
