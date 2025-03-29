use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::models::Wager;

use super::WagerRepository;

pub struct PostgresWagerRepository {
    pool: Arc<PgPool>,
}

impl PostgresWagerRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WagerRepository for PostgresWagerRepository {
    async fn insert_wagers(&self, wagers: Vec<Wager>) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        for wager in wagers {
            sqlx::query(
                r#"
                INSERT INTO jackpot.wagers (
                    id, site_id, game_id, user_id, amount
                ) VALUES ($1, $2, $3, $4, $5)
                "#,
            )
            .bind(wager.id.to_string())
            .bind(wager.site_id)
            .bind(wager.game_id)
            .bind(&wager.user_id)
            .bind(wager.amount)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }
}
