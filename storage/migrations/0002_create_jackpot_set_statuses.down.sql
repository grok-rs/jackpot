-- Down Migration: drop_jackpot_set_statuses.sql
DROP TABLE IF EXISTS jackpot.jackpot_set_statuses;

DROP TYPE IF EXISTS jackpot.status_enum;
