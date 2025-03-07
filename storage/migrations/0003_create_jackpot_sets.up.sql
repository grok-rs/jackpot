-- Up Migration: create_jackpot_sets.sql
CREATE TABLE IF NOT EXISTS jackpot.jackpot_sets (
    id SERIAL PRIMARY KEY,
    wager_share DECIMAL(2, 2),
    status_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_status FOREIGN KEY (status_id) REFERENCES jackpot.jackpot_set_statuses (id) ON DELETE CASCADE
);
