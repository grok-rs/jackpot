use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WagerRequest {
    pub amount: u64,
    pub site_id: u32,
    pub user_id: u32,
    pub game_id: u32,

    pub cheat_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WagerResponse {
    pub wager_id: String,
    pub status: String,
    pub amount: u64,

    pub receipt_id: Option<String>,
}
