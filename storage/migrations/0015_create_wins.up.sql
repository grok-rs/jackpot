CREATE TABLE IF NOT EXISTS jackpot.wins (
    id SERIAL PRIMARY KEY,
    cycle_id INTEGER,
    seed_value DECIMAL(18, 2),
    cycle_started TIMESTAMP,
    cycle_completed TIMESTAMP,
    win_value DECIMAL(18, 2),
    winner_player_id VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_cycle
        FOREIGN KEY (cycle_id)
        REFERENCES jackpot.cycles (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
