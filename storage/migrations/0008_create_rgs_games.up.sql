-- Up Migration: create_rgs_games.sql
CREATE TABLE IF NOT EXISTS jackpot.rgs_games (
    id SERIAL PRIMARY KEY,
    rgs_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    CONSTRAINT fk_rgs FOREIGN KEY (rgs_id) REFERENCES jackpot.rgs (id) ON UPDATE CASCADE ON DELETE CASCADE
);
