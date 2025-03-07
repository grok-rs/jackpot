CREATE TABLE IF NOT EXISTS jackpot.cycles (
    id SERIAL PRIMARY KEY,
    level_id INTEGER NOT NULL,
    win_id INTEGER,
    balance_start DECIMAL(18, 2),
    balance_end DECIMAL(18, 2),
    cycle_start TIMESTAMP,
    cycle_end TIMESTAMP,
    wager_count INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_level
        FOREIGN KEY (level_id)
        REFERENCES jackpot.levels (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
