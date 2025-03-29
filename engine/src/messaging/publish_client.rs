use anyhow::Context;
use lapin::{
    BasicProperties, Channel, ExchangeKind,
    options::{BasicPublishOptions, ExchangeDeclareOptions},
    types::FieldTable,
};
use std::sync::Arc;

use super::connection::RabbitConnection;

pub struct PublishClient {
    channel: Arc<Channel>,
    exchange_name: String,
}

impl PublishClient {
    pub async fn new(
        connection: &RabbitConnection,
        exchange_name: &str,
        exchange_kind: ExchangeKind,
    ) -> anyhow::Result<Self> {
        let channel = Arc::new(connection.create_channel().await?);

        // Declare the exchange
        channel
            .exchange_declare(
                exchange_name,
                exchange_kind,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to declare exchange")?;

        Ok(Self {
            channel,
            exchange_name: exchange_name.to_string(),
        })
    }

    /// Publishes a message to the specified exchange with an optional priority.
    pub async fn publish(&self, message: &str, priority: Option<u8>) -> anyhow::Result<()> {
        let mut props = BasicProperties::default();
        if let Some(p) = priority {
            props = props.with_priority(p);
        }

        self.channel
            .basic_publish(
                &self.exchange_name,
                "", // Empty routing key (can be parameterized if needed)
                BasicPublishOptions::default(),
                message.as_bytes(),
                props,
            )
            .await
            .context("Failed to publish message")?
            .await
            .context("Failed to confirm publish")?;

        Ok(())
    }
}
