-- Zone definitions.
-- EQEmu: zone table (92 columns). ADIF: modernized with JSONB for fog/weather.

CREATE TABLE zones (
    id              SERIAL PRIMARY KEY,
    short_name      VARCHAR(32) UNIQUE NOT NULL,
    long_name       VARCHAR(128) NOT NULL,

    -- Safe/spawn point
    safe_x          REAL NOT NULL DEFAULT 0,
    safe_y          REAL NOT NULL DEFAULT 0,
    safe_z          REAL NOT NULL DEFAULT 0,
    safe_heading    REAL NOT NULL DEFAULT 0,

    -- Boundaries
    underworld_z    REAL NOT NULL DEFAULT -1000,
    max_z           REAL NOT NULL DEFAULT 10000,
    min_clip        REAL NOT NULL DEFAULT 50,
    max_clip        REAL NOT NULL DEFAULT 800,

    -- Physics
    gravity         REAL NOT NULL DEFAULT 0.4,
    sky_type        SMALLINT NOT NULL DEFAULT 1,

    -- Atmosphere (JSONB replaces EQEmu's 20+ fog/weather columns)
    fog             JSONB NOT NULL DEFAULT '[]',
    weather         JSONB NOT NULL DEFAULT '{"rain": [], "snow": []}',

    -- Rules
    can_bind        BOOLEAN NOT NULL DEFAULT FALSE,
    can_combat      BOOLEAN NOT NULL DEFAULT TRUE,
    can_levitate    BOOLEAN NOT NULL DEFAULT TRUE,
    can_cast_outdoor BOOLEAN NOT NULL DEFAULT TRUE,
    exp_multiplier  REAL NOT NULL DEFAULT 1.0,
    max_clients     INTEGER NOT NULL DEFAULT 200,
    min_level       SMALLINT NOT NULL DEFAULT 0,
    min_status      SMALLINT NOT NULL DEFAULT 0,

    -- Geometry type (mesh or voxel — pluggable zones!)
    geometry_type   VARCHAR(16) NOT NULL DEFAULT 'mesh',

    -- Metadata
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Zone transitions.
-- EQEmu: zone_points (21 columns). ADIF: proper foreign keys.

CREATE TABLE zone_points (
    id              SERIAL PRIMARY KEY,
    zone_id         INTEGER NOT NULL REFERENCES zones(id) ON DELETE CASCADE,
    number          SMALLINT NOT NULL DEFAULT 0,

    -- Source position (where the zone line is)
    x               REAL NOT NULL,
    y               REAL NOT NULL,
    z               REAL NOT NULL,
    heading         REAL NOT NULL DEFAULT 0,
    width           REAL NOT NULL DEFAULT 5,
    height          REAL NOT NULL DEFAULT 10,

    -- Destination
    target_zone_id  INTEGER NOT NULL REFERENCES zones(id),
    target_x        REAL NOT NULL DEFAULT 0,
    target_y        REAL NOT NULL DEFAULT 0,
    target_z        REAL NOT NULL DEFAULT 0,
    target_heading  REAL NOT NULL DEFAULT 0
);

CREATE INDEX idx_zone_points_zone ON zone_points (zone_id);
