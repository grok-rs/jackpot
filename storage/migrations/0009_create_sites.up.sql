-- Up Migration: create_sites.sql

CREATE TABLE IF NOT EXISTS jackpot.sites (
    id SERIAL PRIMARY KEY,
    operator_id INTEGER NOT NULL,
    url VARCHAR(255) CHECK (url ~* '^https?://'),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_operator
        FOREIGN KEY (operator_id)
        REFERENCES jackpot.operators (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);