-- Factions.
CREATE TABLE factions (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(128) NOT NULL
);

CREATE TABLE npc_faction_associations (
    npc_faction_id  INTEGER NOT NULL,
    faction_id      INTEGER NOT NULL REFERENCES factions(id) ON DELETE CASCADE,
    value           INTEGER NOT NULL DEFAULT 0,      -- base faction hit/reward
    PRIMARY KEY (npc_faction_id, faction_id)
);

-- Guilds.
CREATE TABLE guilds (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(64) UNIQUE NOT NULL,
    leader_id       INTEGER,
    motd            TEXT NOT NULL DEFAULT '',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Guild ranks are stored on the characters table (guild_id, guild_rank).
-- No separate guild_members table needed — query characters WHERE guild_id = X.

-- Merchants.
CREATE TABLE merchant_lists (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(128) NOT NULL DEFAULT ''
);

CREATE TABLE merchant_entries (
    merchant_id     INTEGER NOT NULL REFERENCES merchant_lists(id) ON DELETE CASCADE,
    item_id         INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    slot            SMALLINT NOT NULL DEFAULT 0,
    quantity        INTEGER NOT NULL DEFAULT -1,      -- -1 = unlimited
    PRIMARY KEY (merchant_id, slot)
);

-- Doors and world objects.
CREATE TABLE doors (
    id              SERIAL PRIMARY KEY,
    zone_id         INTEGER NOT NULL REFERENCES zones(id) ON DELETE CASCADE,
    name            VARCHAR(64) NOT NULL DEFAULT '',
    x               REAL NOT NULL,
    y               REAL NOT NULL,
    z               REAL NOT NULL,
    heading         REAL NOT NULL DEFAULT 0,
    open_type       SMALLINT NOT NULL DEFAULT 0,
    key_item_id     INTEGER NOT NULL DEFAULT 0,
    size            REAL NOT NULL DEFAULT 1.0,
    is_open         BOOLEAN NOT NULL DEFAULT FALSE,
    dest_zone_id    INTEGER,
    dest_x          REAL NOT NULL DEFAULT 0,
    dest_y          REAL NOT NULL DEFAULT 0,
    dest_z          REAL NOT NULL DEFAULT 0,
    dest_heading    REAL NOT NULL DEFAULT 0
);

CREATE INDEX idx_doors_zone ON doors (zone_id);
