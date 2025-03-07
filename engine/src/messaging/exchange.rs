use anyhow::Context;
use lapin::{options::ExchangeDeclareOptions, types::FieldTable, Channel, ExchangeKind};
use std::sync::Arc;

pub struct ExchangeManager;

impl ExchangeManager {
    pub async fn declare(
        channel: Arc<Channel>,
        name: &str,
        kind: ExchangeKind,
    ) -> anyhow::Result<()> {
        channel
            .exchange_declare(
                name,
                kind,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to declare exchange")
    }
}
