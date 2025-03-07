CREATE TYPE jackpot.receipt_status_enum AS ENUM ('PENDING', 'COMPLETED');

CREATE TABLE IF NOT EXISTS jackpot.receipts (
    id SERIAL PRIMARY KEY,
    status jackpot.receipt_status_enum NOT NULL DEFAULT 'PENDING',
    win_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_win
        FOREIGN KEY (win_id)
        REFERENCES jackpot.wins (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
