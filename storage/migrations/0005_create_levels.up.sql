-- Add up migration script here
-- Up Migration: create_levels.sql
CREATE TABLE IF NOT EXISTS jackpot.levels (
    id SERIAL PRIMARY KEY,
    wager_min DECIMAL(18, 2),
    wager_max DECIMAL(18, 2),
    contribution_share_seed DECIMAL(2, 2),
    seed DECIMAL(18, 2),
    contribution_share DECIMAL(18, 2),
    expected_payout DECIMAL(18, 2),
    minimum_payout DECIMAL(18, 2),
    transfer_seed BOOLEAN,
    jackpot_set_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_jackpot_set FOREIGN KEY (jackpot_set_id) REFERENCES jackpot.jackpot_sets (id) ON UPDATE CASCADE ON DELETE CASCADE
);
