CREATE TABLE IF NOT EXISTS jackpot.game_profiles (
    id SERIAL PRIMARY KEY,
    rgs_game_id INTEGER NOT NULL,
    site_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_rgs_game
        FOREIGN KEY (rgs_game_id)
        REFERENCES jackpot.rgs_games (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    CONSTRAINT fk_site
        FOREIGN KEY (site_id)
        REFERENCES jackpot.sites (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
