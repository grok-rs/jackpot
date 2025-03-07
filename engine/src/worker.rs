use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions},
    types::FieldTable,
};
use std::sync::Arc;
use tracing::{error, info, instrument};

use crate::{domain::models::WagerRequest, services::processor::JackpotProcessor};

#[instrument(name = "start_worker", skip(gateway_channel, processor))]
pub async fn start_worker(
    gateway_channel: lapin::Channel,
    processor: Arc<JackpotProcessor>,
) -> anyhow::Result<()> {
    info!("Starting worker to consume from gateway_queue");

    let mut consumer = gateway_channel
        .basic_consume(
            "gateway_queue",
            "jackpot_engine_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let request: WagerRequest = match serde_json::from_slice(&delivery.data) {
                Ok(req) => req,
                Err(e) => {
                    error!("Failed to deserialize request: {:?}", e);
                    continue;
                }
            };

            let processor = processor.clone();
            let gateway_channel = gateway_channel.clone();

            // Spawn an async task with instrumentation
            tokio::spawn(async move {
                info!("Processing delivery");

                match processor.process_wager(request).await {
                    Ok(response) => {
                        info!("Wager processed successfully");
                        match serde_json::to_vec(&response) {
                            Ok(response_bytes) => {
                                if let Some(reply_to) = delivery.properties.reply_to() {
                                    if let Err(e) = gateway_channel
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
            });
        }
    }
    Ok(())
}
