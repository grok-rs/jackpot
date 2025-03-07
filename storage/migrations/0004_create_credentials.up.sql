-- Up Migration: create_credentials.sql
CREATE TABLE IF NOT EXISTS jackpot.credentials (
    id SERIAL PRIMARY KEY,
    realm VARCHAR(255),
    entity_type VARCHAR(255),
    entity_id INTEGER,
    client_id VARCHAR(36),
    secret VARCHAR(400),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
);
