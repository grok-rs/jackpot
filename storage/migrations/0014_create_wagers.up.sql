CREATE TABLE IF NOT EXISTS jackpot.wagers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id INTEGER NOT NULL,
    rgs_game_id INTEGER NOT NULL,
    player_id VARCHAR(255),
    game_type VARCHAR(255),
    amount DECIMAL(18, 2),
    received_at TIMESTAMP,
    processed_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),

    CONSTRAINT fk_site
        FOREIGN KEY (site_id)
        REFERENCES jackpot.sites (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    CONSTRAINT fk_rgs_game
        FOREIGN KEY (rgs_game_id)
        REFERENCES jackpot.rgs_games (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
