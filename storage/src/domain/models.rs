use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WagerRequest {
    pub amount: f64,
    pub site_id: i32,
    pub user_id: i32,
    pub game_id: i32,

    pub cheat_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WagerResponse {
    pub wager_id: String,
    pub status: String,
    pub amount: f64,

    pub receipt_id: Option<String>,
}
