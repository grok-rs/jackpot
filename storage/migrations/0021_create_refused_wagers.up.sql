CREATE TABLE IF NOT EXISTS jackpot.refused_wagers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id INTEGER,
    rgs_game_id INTEGER,
    reason VARCHAR(255),
    player_id VARCHAR(255),
    game_type VARCHAR(255),
    amount DECIMAL(18, 2),
    received_at TIMESTAMP,
    processed_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);
