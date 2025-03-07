CREATE TABLE IF NOT EXISTS jackpot.balance_sheet (
    id SERIAL PRIMARY KEY,
    wager_id UUID,
    cycle_id INTEGER NOT NULL,
    wager_order INTEGER,
    player_id VARCHAR(255),
    site_id INTEGER NOT NULL,
    jackpot_id INTEGER NOT NULL,
    jackpot_before DECIMAL(18, 2),
    jackpot_after DECIMAL(18, 2),
    wager_amount DECIMAL(18, 2),
    winning_amount DECIMAL(18, 2),
    wager_share DECIMAL(18, 2),
    interconnected BOOLEAN,
    amount_free_game INTEGER,
    received_at TIMESTAMP,
    processed_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),

    CONSTRAINT fk_cycle
        FOREIGN KEY (cycle_id)
        REFERENCES jackpot.cycles (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    CONSTRAINT fk_site
        FOREIGN KEY (site_id)
        REFERENCES jackpot.sites (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    CONSTRAINT fk_jackpot_level
        FOREIGN KEY (jackpot_id)
        REFERENCES jackpot.levels (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
