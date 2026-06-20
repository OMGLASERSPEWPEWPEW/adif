-- Accounts and authentication.
-- EQEmu: account table (24 columns). ADIF: cleaner, proper types.

CREATE TABLE accounts (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(64) UNIQUE NOT NULL,
    password_hash   VARCHAR(255) NOT NULL,
    email           VARCHAR(255),
    status          SMALLINT NOT NULL DEFAULT 0,  -- 0=player, 200+=GM, 250=admin
    is_banned       BOOLEAN NOT NULL DEFAULT FALSE,
    ban_reason      TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at   TIMESTAMPTZ,
    login_count     INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_accounts_name ON accounts (name);
