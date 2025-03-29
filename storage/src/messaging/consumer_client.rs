use anyhow::Context;
use futures::StreamExt;
use lapin::{
    Channel, ExchangeKind,
    message::Delivery,
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, ExchangeDeclareOptions,
        QueueBindOptions, QueueDeclareOptions,
    },
    types::FieldTable,
};
use std::sync::Arc;
use tracing::{error, info};

use super::connection::RabbitConnection;
use crate::domain::models::WagerRequest;
use crate::services::storage_processor::TrunsatictionProcessor;

pub struct ConsumerClient {
    channel: Arc<Channel>,
    queue_name: String,
    processor: Arc<TrunsatictionProcessor>,
}

impl ConsumerClient {
    /// Creates a new `ConsumerClient`, setting up the channel, exchange, queue, and binding.
    pub async fn new(
        connection: &RabbitConnection,
        exchange_name: &str,
        exchange_kind: ExchangeKind,
        queue_name: &str,
        routing_key: &str,
        processor: Arc<TrunsatictionProcessor>,
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

        // Declare the queue
        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to declare queue")?;

        // Bind the queue to the exchange
        channel
            .queue_bind(
                queue_name,
                exchange_name,
                routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to bind queue")?;

        Ok(Self {
            channel,
            queue_name: queue_name.to_string(),
            processor,
        })
    }

    /// Starts consuming messages from the queue, processing each in a spawned task.
    pub async fn start_consuming(&self) -> anyhow::Result<()> {
        info!("Starting consumer for queue: {}", self.queue_name);

        let mut consumer = self
            .channel
            .basic_consume(
                &self.queue_name,
                "storage_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to start consumer")?;

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let client = self.clone();
                tokio::spawn(async move {
                    client.process_delivery(delivery).await;
                });
            }
        }
        Ok(())
    }

    /// Processes a single delivery: deserializes, processes, sends response, and acknowledges.
    async fn process_delivery(&self, delivery: Delivery) {
        info!("Processing delivery");

        let request: WagerRequest = match serde_json::from_slice(&delivery.data) {
            Ok(req) => req,
            Err(e) => {
                error!("Failed to deserialize request: {:?}", e);
                // Acknowledge the message even on failure to prevent redelivery
                // TODO write wrong request to DB
                if let Err(ack_err) = delivery.ack(BasicAckOptions::default()).await {
                    error!("Failed to acknowledge message: {:?}", ack_err);
                }
                return;
            }
        };

        match self.processor.process_wager(request).await {
            Ok(response) => {
                info!("Wager processed successfully");
                match serde_json::to_vec(&response) {
                    Ok(response_bytes) => {
                        if let Some(reply_to) = delivery.properties.reply_to() {
                            if let Err(e) = self
                                .channel
                                .basic_publish(
                                    "",
                                    reply_to.as_str(),
                                    BasicPublishOptions::default(),
                                    &response_bytes,
                                    lapin::BasicProperties::default().with_correlation_id(
                                        delivery
                                            .properties
                                            .correlation_id()
                                            .clone()
                                            .unwrap_or_default(),
                                    ),
                                )
                                .await
                            {
                                error!("Failed to send response: {:?}", e);
                            } else {
                                info!("Response sent to reply_to queue");
                            }
                        }
                        if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                            error!("Failed to acknowledge message: {:?}", e);
                        } else {
                            info!("Message acknowledged");
                        }
                    }
                    Err(e) => {
                        error!("Failed to serialize response: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to process wager: {:?}", e);
            }
        }
    }
}

impl Clone for ConsumerClient {
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.clone(),
            queue_name: self.queue_name.clone(),
            processor: self.processor.clone(),
        }
    }
}
