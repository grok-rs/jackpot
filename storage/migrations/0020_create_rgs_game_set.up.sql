CREATE TABLE IF NOT EXISTS jackpot.rgs_game_set (
    id SERIAL PRIMARY KEY,
    game_id INTEGER NOT NULL,
    jackpot_set_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_rgs_game
        FOREIGN KEY (game_id)
        REFERENCES jackpot.rgs_games (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    CONSTRAINT fk_jackpot_set
        FOREIGN KEY (jackpot_set_id)
        REFERENCES jackpot.jackpot_sets (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
