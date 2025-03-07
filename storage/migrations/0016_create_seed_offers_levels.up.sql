CREATE TABLE IF NOT EXISTS jackpot.seed_offers_levels (
    id SERIAL PRIMARY KEY,
    seed_amount INTEGER,
    seed_id INTEGER NOT NULL,
    level_id INTEGER NOT NULL,

    CONSTRAINT fk_seed_offer
        FOREIGN KEY (seed_id)
        REFERENCES jackpot.seed_offers (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    CONSTRAINT fk_level
        FOREIGN KEY (level_id)
        REFERENCES jackpot.levels (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
