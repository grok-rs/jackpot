use std::sync::Arc;

use anyhow::Context;
use lapin::{options::ExchangeDeclareOptions, types::FieldTable, Channel, ExchangeKind};

pub struct ExchangeManager {
    channel: Arc<Channel>,
    name: String,
}

impl ExchangeManager {
    pub async fn declare(
        channel: Arc<Channel>,
        name: &str,
        kind: ExchangeKind,
    ) -> anyhow::Result<Self> {
        channel
            .exchange_declare(
                name,
                kind,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to declare exchange")?;

        Ok(Self {
            channel,
            name: name.to_string(),
        })
    }
}
