CREATE TABLE IF NOT EXISTS jackpot.seed_offers (
    id SERIAL PRIMARY KEY,
    site_id INTEGER NOT NULL,

    CONSTRAINT fk_site
        FOREIGN KEY (site_id)
        REFERENCES jackpot.sites (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
