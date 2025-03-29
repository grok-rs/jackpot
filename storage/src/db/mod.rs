use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub mod wager_repository;

pub struct DatabaseClient {
    pool: Arc<Pool<Postgres>>,
}

impl DatabaseClient {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

use async_trait::async_trait;

use crate::domain::models::Wager;

#[async_trait]
pub trait WagerRepository {
    async fn insert_wagers(&self, wagers: Vec<Wager>) -> anyhow::Result<()>;
}
