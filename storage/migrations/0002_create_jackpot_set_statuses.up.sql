-- Up Migration: create_jackpot_set_statuses.sql
CREATE TYPE jackpot.status_enum AS ENUM ('PENDING', 'ACTIVE', 'INTERRUPTED', 'COMPLETED');

CREATE TABLE IF NOT EXISTS jackpot.jackpot_set_statuses (
    id SERIAL PRIMARY KEY,
    status_name jackpot.status_enum NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
);
