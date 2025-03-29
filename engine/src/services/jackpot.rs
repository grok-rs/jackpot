use crate::domain::models::WagerRequest;
use redis::aio::ConnectionManager;

pub struct JackpotService {
    redis: ConnectionManager,
}

impl JackpotService {
    pub async fn new(redis_url: &str) -> anyhow::Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;
        Ok(Self { redis })
    }

    pub async fn update_balance_and_check_win(
        &self,
        request: &WagerRequest,
    ) -> anyhow::Result<(bool, u64)> {
        let won = rand::random::<bool>(); // TODO Replace with certified RNG logic
        let new_balance = if won { 0 } else { request.amount };
        Ok((won, new_balance))
    }
}
