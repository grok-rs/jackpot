use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::{Context, anyhow};
use futures::StreamExt;
use lapin::{
    BasicProperties, Channel, ExchangeKind,
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::{FieldTable, ShortString},
};
use serde::de::DeserializeOwned;
use tokio::sync::{Mutex, oneshot};
use tracing::{error, instrument};
use uuid::Uuid;

use crate::domain::models::WagerRequest;

use super::connection::RabbitConnection;

pub struct RpcClient<Response> {
    channel: Arc<Channel>,
    exchange_name: String,
    reply_queue_name: String,
    pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<Response>>>>,
}

impl<Response> RpcClient<Response>
where
    Response: DeserializeOwned + Send + 'static,
{
    pub async fn new(
        connection: &RabbitConnection,
        exchange_name: &str,
        exchange_kind: ExchangeKind,
    ) -> anyhow::Result<Self> {
        let channel = Arc::new(connection.create_channel().await?);

        // Declare the exchange directly (migrated from ExchangeManager)
        channel
            .exchange_declare(
                exchange_name,
                exchange_kind,
                lapin::options::ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .context("Failed to declare exchange")?;

        // Declare the reply queue directly
        let queue = channel
            .queue_declare(
                "",
                QueueDeclareOptions {
                    exclusive: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .context("Failed to create reply queue")?;
        let reply_queue_name = queue.name().as_str().to_string();

        let pending_requests = Arc::new(Mutex::new(HashMap::new()));

        let client = Self {
            channel,
            exchange_name: exchange_name.to_string(),
            reply_queue_name,
            pending_requests,
        };

        client.spawn_reply_consumer();

        Ok(client)
    }

    #[instrument(name = "rpc.send_request", skip(self, request))]
    pub async fn call(&self, request: &WagerRequest) -> anyhow::Result<Response> {
        let correlation_id = Uuid::new_v4().to_string();
        let (tx, rx) = oneshot::channel();

        // Proper async lock handling
        self.pending_requests
            .lock()
            .await
            .insert(correlation_id.clone(), tx);

        let payload = serde_json::to_vec(&request)?;
        self.channel
            .basic_publish(
                &self.exchange_name,
                "",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
                    .with_correlation_id(ShortString::from(correlation_id.clone()))
                    .with_reply_to(ShortString::from(self.reply_queue_name.clone())),
            )
            .await?
            .await?;

        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(res)) => Ok(res),
            Ok(Err(_)) => Err(anyhow!("Response channel closed")),
            Err(_) => Err(anyhow!("RPC timeout")),
        }
    }

    fn spawn_reply_consumer(&self) {
        let channel = self.channel.clone();
        let queue_name = self.reply_queue_name.clone();
        let pending_requests = self.pending_requests.clone();
        tokio::spawn(async move {
            let mut consumer = match channel
                .basic_consume(
                    &queue_name,
                    "rpc_consumer",
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await
            {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to create consumer: {}", e);
                    return;
                }
            };

            while let Some(delivery) = consumer.next().await {
                match delivery {
                    Ok(delivery) => {
                        let corr_id = delivery.properties.correlation_id().clone();

                        // Handle ack first
                        if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                            error!("Failed to ack message: {}", e);
                        }

                        if let Some(corr_id) = corr_id {
                            let corr_str = corr_id.as_str().to_string();

                            // Handle response processing
                            let tx = match pending_requests.lock().await.remove(&corr_str) {
                                Some(tx) => tx,
                                None => {
                                    error!("No pending request for correlation ID: {}", corr_str);
                                    continue;
                                }
                            };

                            match serde_json::from_slice::<Response>(&delivery.data) {
                                Ok(response) => {
                                    if let Err(_) = tx.send(response) {
                                        error!("Failed to send response");
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to deserialize response: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => error!("Delivery error: {}", e),
                }
            }
        });
    }
}
