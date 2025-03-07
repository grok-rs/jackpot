use anyhow::anyhow;
use futures::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions},
    types::{FieldTable, ShortString},
    BasicProperties, Channel,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::{oneshot, Mutex};
use tracing::{error, instrument};
use uuid::Uuid;

use super::{connection::RabbitConnection, exchange::ExchangeManager, reply_queue::ReplyQueue};

pub struct RpcClient<Response> {
    channel: Arc<Channel>,
    exchange_name: String,
    reply_queue: ReplyQueue,
    pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<Response>>>>,
}

impl<Response> RpcClient<Response>
where
    Response: DeserializeOwned + Send + 'static,
{
    pub async fn new(
        connection: &RabbitConnection,
        exchange_name: &str,
        exchange_kind: lapin::ExchangeKind,
    ) -> anyhow::Result<Self> {
        let channel = Arc::new(connection.create_channel().await?);
        ExchangeManager::declare(Arc::clone(&channel), exchange_name, exchange_kind).await?;
        let reply_queue = ReplyQueue::new(Arc::clone(&channel)).await?;

        let pending_requests = Arc::new(Mutex::new(HashMap::new()));
        Self::spawn_reply_consumer(Arc::clone(&channel), &reply_queue, pending_requests.clone());

        Ok(Self {
            channel,
            exchange_name: exchange_name.to_string(),
            reply_queue,
            pending_requests,
        })
    }

    #[instrument(name = "rpc.send_request", skip(self, request))]
    pub async fn call<T: Serialize>(&self, request: &T) -> anyhow::Result<Response> {
        let correlation_id = Uuid::new_v4().to_string();
        let (tx, rx) = oneshot::channel();

        self.pending_requests
            .lock()
            .await
            .insert(correlation_id.clone(), tx);

        let payload = serde_json::to_vec(request)?;
        self.channel
            .basic_publish(
                &self.exchange_name,
                "",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
                    .with_correlation_id(ShortString::from(correlation_id.clone()))
                    .with_reply_to(ShortString::from(self.reply_queue.name().to_string())),
            )
            .await?
            .await?;

        match tokio::time::timeout(Duration::from_secs(5), rx).await {
            Ok(Ok(res)) => Ok(res),
            Ok(Err(_)) => Err(anyhow!("Response channel closed")),
            Err(_) => Err(anyhow!("RPC timeout")),
        }
    }

    fn spawn_reply_consumer(
        channel: Arc<Channel>,
        reply_queue: &ReplyQueue,
        pending_requests: Arc<Mutex<HashMap<String, oneshot::Sender<Response>>>>,
    ) {
        let queue_name = reply_queue.name().to_string();
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
                        if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                            error!("Failed to ack message: {}", e);
                        }

                        if let Some(corr_id) = corr_id {
                            let corr_str = corr_id.as_str().to_string();
                            if let Some(tx) = pending_requests.lock().await.remove(&corr_str) {
                                match serde_json::from_slice::<Response>(&delivery.data) {
                                    Ok(response) => {
                                        if tx.send(response).is_err() {
                                            error!("Failed to send response");
                                        }
                                    }
                                    Err(e) => error!("Deserialization failed: {}", e),
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
