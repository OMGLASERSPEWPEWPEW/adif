-- 019_guild_extensions.sql
-- Guild membership and rank system.
--
-- EQEmu: guild_members, guild_ranks stored separately from the guilds table.
-- ADIF guilds table (007_factions_and_guilds.sql) stores the guild itself;
-- these tables store membership and rank configuration.

-- Guild membership (which character belongs to which guild)
-- EQEmu: guild_members. Separate from characters table.
CREATE TABLE IF NOT EXISTS guild_members (
    char_id         INTEGER PRIMARY KEY,
    guild_id        INTEGER NOT NULL DEFAULT 0,
    rank            SMALLINT NOT NULL DEFAULT 0,
    tribute_enable  SMALLINT NOT NULL DEFAULT 0,
    total_tribute   INTEGER NOT NULL DEFAULT 0,
    last_tribute    INTEGER NOT NULL DEFAULT 0,
    banker          SMALLINT NOT NULL DEFAULT 0,
    public_note     TEXT NOT NULL DEFAULT '',
    alt             SMALLINT NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_guild_members_guild ON guild_members(guild_id);

-- Guild rank definitions and permissions
-- EQEmu: guild_ranks. Per-guild rank configuration.
CREATE TABLE IF NOT EXISTS guild_ranks (
    guild_id    INTEGER NOT NULL DEFAULT 0,
    rank        SMALLINT NOT NULL DEFAULT 0,
    title       TEXT NOT NULL DEFAULT '',
    can_hear    SMALLINT NOT NULL DEFAULT 0,
    can_speak   SMALLINT NOT NULL DEFAULT 0,
    can_invite  SMALLINT NOT NULL DEFAULT 0,
    can_remove  SMALLINT NOT NULL DEFAULT 0,
    can_promote SMALLINT NOT NULL DEFAULT 0,
    can_demote  SMALLINT NOT NULL DEFAULT 0,
    can_motd    SMALLINT NOT NULL DEFAULT 0,
    can_warpeace SMALLINT NOT NULL DEFAULT 0,
    PRIMARY KEY (guild_id, rank)
);
