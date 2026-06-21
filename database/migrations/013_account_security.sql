-- 013_account_security.sql
-- Account extension tables for security, IP tracking, and bans.
--
-- EQEmu: account_flags, account_ip, banned_ips, gm_ips.
-- ADIF: Identical — security infrastructure is stack-agnostic.

-- Per-account feature flags (veteran rewards, special access, etc.)
-- EQEmu: account_flags. Unchanged.
CREATE TABLE IF NOT EXISTS account_flags (
    id        SERIAL PRIMARY KEY,
    accid     INTEGER NOT NULL DEFAULT 0,
    flag_name TEXT NOT NULL DEFAULT '',
    flag_value TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_account_flags_accid ON account_flags(accid);

-- IP address tracking per account (login history)
-- EQEmu: account_ip. Unchanged.
CREATE TABLE IF NOT EXISTS account_ip (
    id       SERIAL PRIMARY KEY,
    accid    INTEGER NOT NULL DEFAULT 0,
    ip       TEXT NOT NULL DEFAULT '',
    count    INTEGER NOT NULL DEFAULT 0,
    lastused TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_account_ip_accid ON account_ip(accid);

-- Account veteran reward tracking
-- EQEmu: account_rewards. Simple reward claim tracking.
CREATE TABLE IF NOT EXISTS account_rewards (
    account_id INTEGER NOT NULL DEFAULT 0,
    reward_id  INTEGER NOT NULL DEFAULT 0,
    amount     INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (account_id, reward_id)
);

-- Banned IP addresses
-- EQEmu: banned_ips. Unchanged.
CREATE TABLE IF NOT EXISTS banned_ips (
    id         SERIAL PRIMARY KEY,
    ip_address TEXT NOT NULL DEFAULT '',
    notes      TEXT NOT NULL DEFAULT ''
);

-- GM-allowed IP addresses (whitelist)
-- EQEmu: gm_ips. Unchanged.
CREATE TABLE IF NOT EXISTS gm_ips (
    id         SERIAL PRIMARY KEY,
    account_id INTEGER NOT NULL DEFAULT 0,
    ip_address TEXT NOT NULL DEFAULT '',
    name       TEXT NOT NULL DEFAULT ''
);
CREATE INDEX IF NOT EXISTS idx_gm_ips_account ON gm_ips(account_id);
