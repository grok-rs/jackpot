use std::sync::Arc;
use tracing::instrument;

use crate::domain::models::{Wager, WagerResponse};

use super::storage::StorageService;

pub struct TrunsatictionProcessor {
    pub storage_service: Arc<StorageService>,
}

impl TrunsatictionProcessor {
    #[instrument(name = "process_wager", skip(self, request), fields(user_id = %request.user_id, amount = request.amount))]
    pub async fn process_wager(&self, request: Wager) -> anyhow::Result<WagerResponse> {
        tracing::info!("Starting wager processing");

        let response = WagerResponse {
            wager_id: "aa".to_string(),
            status: "Test".to_string(),
            amount: 10.0, // Note: Hardcoded value from original code; consider using request.amount
            receipt_id: Some("123".to_string()),
        };

        Ok(response)
    }
}
