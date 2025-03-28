use anyhow::{Context, Result};
use futures::StreamExt;
use lapin::{
    Connection, ConnectionProperties, Consumer, ExchangeKind, options::*, types::FieldTable,
};

pub struct RabbitMQConsumer {
    consumer: Consumer,
}

impl RabbitMQConsumer {
    pub async fn new(
        uri: &str,
        exchange_name: &str,
        exchange_kind: ExchangeKind,
        queue_name: &str,
        routing_key: &str,
    ) -> Result<Self> {
        let connection = Connection::connect(uri, ConnectionProperties::default())
            .await
            .context("Failed to connect to RabbitMQ")?;
        let channel = connection.create_channel().await?;

        // Declare the exchange (same as gateway)
        channel
            .exchange_declare(
                exchange_name,
                exchange_kind,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to declare exchange")?;

        // Declare the queue
        let queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to declare queue")?;

        // Bind queue to exchange with routing key
        channel
            .queue_bind(
                queue.name().as_str(),
                exchange_name,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to bind queue")?;

        // Create consumer
        let consumer = channel
            .basic_consume(
                queue.name().as_str(),
                "engine_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to create consumer")?;

        Ok(Self { consumer })
    }

    pub async fn consume(&mut self) -> Result<Option<lapin::message::Delivery>> {
        match self.consumer.next().await {
            Some(Ok(delivery)) => Ok(Some(delivery)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }
}
