use anyhow::{Context, Result};
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
    ExchangeKind,
};

pub struct RabbitMQPublisher {
    channel: Channel,
    exchange_name: String,
}

impl RabbitMQPublisher {
    pub async fn new(uri: &str, exchange_name: &str, exchange_kind: ExchangeKind) -> Result<Self> {
        let connection = Connection::connect(uri, ConnectionProperties::default())
            .await
            .context("Failed to connect to RabbitMQ")?;

        let channel = connection.create_channel().await?;

        // Declare the same exchange as gateway
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

    pub async fn publish_rpc_response(
        &self,
        routing_key: &str,
        message: &[u8],
        correlation_id: Option<String>,
    ) -> Result<()> {
        self.channel
            .basic_publish(
                &self.exchange_name,
                routing_key,
                BasicPublishOptions::default(),
                message,
                BasicProperties::default()
                    .with_correlation_id(correlation_id.map(|id| id.into()).unwrap_or_default()),
            )
            .await?
            .await
            .context("Failed to publish message")?;

        Ok(())
    }
}
