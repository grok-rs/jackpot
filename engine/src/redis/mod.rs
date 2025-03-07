use anyhow::Result;
use redis::{aio::Connection, AsyncCommands, Script};
use uuid::Uuid;

pub struct RedisBalanceRepository {
    conn: Connection,
}

impl RedisBalanceRepository {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }
}

impl super::domain::BalanceRepository for RedisBalanceRepository {
    fn update_balance(&self, user_id: Uuid, amount: f64) -> Result<f64> {
        let lua_script = Script::new(
            r#"
            local current = redis.call('GET', KEYS[1])
            current = tonumber(current) or 0
            local new_balance = current + tonumber(ARGV[1])
            redis.call('SET', KEYS[1], new_balance)
            return new_balance
            "#,
        );

        let key = format!("balance:{}", user_id);
        let result: f64 = lua_script
            .key(key)
            .arg(amount)
            .invoke_async(&mut self.conn.clone())
            .await?;

        Ok(result)
    }
}
