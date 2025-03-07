-- Add up migration script here
-- Up Migration: create_admins.sql
CREATE TABLE IF NOT EXISTS jackpot.admins (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE CHECK (POSITION('@' IN email) > 1),
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP DEFAULT NOW ()
);
