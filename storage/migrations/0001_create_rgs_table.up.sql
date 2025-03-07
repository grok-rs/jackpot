-- Up Migration: create_rgs_table.sql
CREATE SCHEMA IF NOT EXISTS jackpot;

CREATE TABLE IF NOT EXISTS jackpot.rgs (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE CHECK (POSITION('@' IN email) > 1),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW ()
);
