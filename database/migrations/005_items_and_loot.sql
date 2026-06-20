-- Items and loot system.
-- EQEmu: items (170+ columns!). ADIF: core columns + JSONB for stats/effects.

CREATE TABLE items (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(128) NOT NULL,
    lore_text       TEXT NOT NULL DEFAULT '',

    -- Classification
    item_class      SMALLINT NOT NULL DEFAULT 0,    -- 0=common, 1=container, 2=book
    item_type       SMALLINT NOT NULL DEFAULT 0,    -- weapon type, armor type, etc.

    -- Restrictions (bitmasks)
    classes         INTEGER NOT NULL DEFAULT 65535,  -- class bitmask (all=65535)
    races           INTEGER NOT NULL DEFAULT 65535,  -- race bitmask (all=65535)
    slots           INTEGER NOT NULL DEFAULT 0,      -- equip slot bitmask

    -- Physical
    weight          SMALLINT NOT NULL DEFAULT 0,     -- in tenths of a pound
    size            SMALLINT NOT NULL DEFAULT 0,     -- 0=tiny, 4=giant
    icon            INTEGER NOT NULL DEFAULT 0,

    -- Combat (if weapon)
    damage          SMALLINT NOT NULL DEFAULT 0,
    delay           SMALLINT NOT NULL DEFAULT 0,
    range           SMALLINT NOT NULL DEFAULT 0,

    -- Defense (if armor)
    ac              SMALLINT NOT NULL DEFAULT 0,

    -- Stats and effects (JSONB replaces EQEmu's 50+ stat columns)
    stats           JSONB NOT NULL DEFAULT '{}',
    -- Example: {"hp": 25, "mana": 15, "str": 5, "magic_resist": 10}

    effects         JSONB NOT NULL DEFAULT '[]',
    -- Example: [{"type": "worn", "spell_id": 123}, {"type": "click", "spell_id": 456, "charges": 3}]

    -- Container (if bag)
    bag_slots       SMALLINT NOT NULL DEFAULT 0,
    bag_size        SMALLINT NOT NULL DEFAULT 0,
    bag_weight_reduction SMALLINT NOT NULL DEFAULT 0,

    -- Flags
    no_trade        BOOLEAN NOT NULL DEFAULT FALSE,
    no_rent         BOOLEAN NOT NULL DEFAULT FALSE,
    magic           BOOLEAN NOT NULL DEFAULT FALSE,
    lore            BOOLEAN NOT NULL DEFAULT FALSE,
    artifact        BOOLEAN NOT NULL DEFAULT FALSE,

    -- Commerce
    price           INTEGER NOT NULL DEFAULT 0,      -- in copper
    sell_rate       REAL NOT NULL DEFAULT 1.0
);

CREATE INDEX idx_items_name ON items (name);

-- Loot tables: what an NPC can drop.
CREATE TABLE loot_tables (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(128) NOT NULL DEFAULT '',
    min_cash        INTEGER NOT NULL DEFAULT 0,      -- copper
    max_cash        INTEGER NOT NULL DEFAULT 0       -- copper
);

-- Loot drops: groups of items within a loot table.
CREATE TABLE loot_drops (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(128) NOT NULL DEFAULT ''
);

-- Which drops belong to which tables.
CREATE TABLE loot_table_entries (
    loot_table_id   INTEGER NOT NULL REFERENCES loot_tables(id) ON DELETE CASCADE,
    loot_drop_id    INTEGER NOT NULL REFERENCES loot_drops(id) ON DELETE CASCADE,
    multiplier      SMALLINT NOT NULL DEFAULT 1,
    drop_limit      SMALLINT NOT NULL DEFAULT 0,
    min_drop        SMALLINT NOT NULL DEFAULT 0,
    probability     REAL NOT NULL DEFAULT 100.0,
    PRIMARY KEY (loot_table_id, loot_drop_id)
);

-- Which items are in which drops.
CREATE TABLE loot_drop_entries (
    loot_drop_id    INTEGER NOT NULL REFERENCES loot_drops(id) ON DELETE CASCADE,
    item_id         INTEGER NOT NULL REFERENCES items(id) ON DELETE CASCADE,
    chance          REAL NOT NULL DEFAULT 100.0,     -- percentage
    min_quantity    SMALLINT NOT NULL DEFAULT 1,
    max_quantity    SMALLINT NOT NULL DEFAULT 1,
    PRIMARY KEY (loot_drop_id, item_id)
);
