use crate::{
    db::{WagerRepository, wager_repository::PostgresWagerRepository},
    domain::models::Wager,
};
use tracing::instrument;

pub struct StorageService {
    wager_repository: PostgresWagerRepository,
}

impl StorageService {
    pub async fn new(wager_repository: PostgresWagerRepository) -> anyhow::Result<Self> {
        Ok(Self { wager_repository })
    }

    #[instrument(skip(self, wagers), fields(wager_count = wagers.len()))]
    pub async fn write_transactions(&self, wagers: Vec<Wager>) -> anyhow::Result<()> {
        self.wager_repository.insert_wagers(wagers).await?;
        Ok(())
    }
}
