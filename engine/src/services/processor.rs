use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    domain::models::{ReceiptResponse, WagerRequest, WagerResponse},
    messaging::{publish_client::PublishClient, rpc_client::RpcClient},
};

use super::jackpot::JackpotService;

pub struct JackpotProcessor {
    pub jackpot_service: Arc<JackpotService>,
    pub storage_rpc_client: Arc<RpcClient<ReceiptResponse>>,
    pub publish_client: Arc<PublishClient>,
}

impl JackpotProcessor {
    #[instrument(name = "process_wager", skip(self, request), fields(user_id = %request.user_id, amount = request.amount))]
    pub async fn process_wager(&self, request: WagerRequest) -> anyhow::Result<WagerResponse> {
        tracing::info!("Starting wager processing");

        let (won, new_balance) = self
            .jackpot_service
            .update_balance_and_check_win(&request)
            .await?;

        tracing::info!(
            won = won,
            new_balance = new_balance,
            "Jackpot result determined"
        );

        let mut response = WagerResponse {
            wager_id: Uuid::new_v4(),
            status: won.to_string(),
            amount: request.amount,
            receipt_id: None,
        };

        if won {
            tracing::info!("Jackpot won, sending RPC to storage with priority");
            let receipt_response = self
                .storage_rpc_client
                .call(&serde_json::to_string(&request)?, Some(10))
                .await?;

            tracing::info!(
                receipt_id = receipt_response.receipt_id,
                "Received receipt ID from storage"
            );
            response.receipt_id = Some(receipt_response.receipt_id);
        } else {
            tracing::info!("Jackpot lost, publishing to storage without priority");
            self.publish_client
                .publish(&serde_json::to_string(&request)?, Some(1))
                .await?;
            tracing::info!("Published loss transaction to storage");
        }

        tracing::info!("Wager processing completed successfully");
        Ok(response)
    }
}
