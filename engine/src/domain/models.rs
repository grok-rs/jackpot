use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct WagerRequest {
    pub id: Option<Uuid>,
    pub amount: u64,
    pub site_id: i32,
    pub user_id: i32,
    pub game_id: i32,

    pub cheat_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WagerResponse {
    #[serde(with = "uuid::serde::compact")]
    pub wager_id: Uuid,
    pub status: String,
    pub amount: u64,

    pub receipt_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ReceiptResponse {
    pub receipt_id: String,
}
