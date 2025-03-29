CREATE SCHEMA IF NOT EXISTS jackpot;

CREATE TABLE IF NOT EXISTS jackpot.wagers (
    id UUID PRIMARY KEY,
    site_id INTEGER NOT NULL,
    game_id INTEGER NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    amount DECIMAL(18, 2) NOT NULL,
    received_at TIMESTAMP DEFAULT NOW (),
    processed_at TIMESTAMP DEFAULT NOW (),
    created_at TIMESTAMP DEFAULT NOW ()
);
