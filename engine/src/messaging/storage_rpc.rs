// use lapin::{
//     BasicProperties,
//     options::{BasicPublishOptions, QueueDeclareOptions},
//     types::FieldTable,
// };
// use std::sync::Arc;
// use tokio::sync::Mutex;
// use tracing::info;
// use uuid::Uuid;

// use crate::messaging::connection::RabbitConnection;

// pub struct StorageRpcClient {
//     channel: Arc<lapin::Channel>,
//     exchange_name: String,
//     reply_queue: String,
//     pending_requests:
//         Arc<Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<String>>>>,
// }

// impl StorageRpcClient {
//     pub async fn new(connection: &RabbitConnection, exchange_name: &str) -> anyhow::Result<Self> {
//         let channel = Arc::new(connection.create_channel().await?);
//         let reply_queue = channel
//             .queue_declare(
//                 "",
//                 QueueDeclareOptions {
//                     exclusive: true,
//                     ..Default::default()
//                 },
//                 FieldTable::default(),
//             )
//             .await?
//             .name()
//             .to_string();

//         let pending_requests = Arc::new(Mutex::new(std::collections::HashMap::new()));

//         Ok(Self {
//             channel,
//             exchange_name: exchange_name.to_string(),
//             reply_queue,
//             pending_requests,
//         })
//     }

//     pub async fn call(&self, message: &str, priority: Option<u8>) -> anyhow::Result<String> {
//         let correlation_id = Uuid::new_v4().to_string();
//         let (tx, rx) = tokio::sync::oneshot::channel();
//         self.pending_requests
//             .lock()
//             .await
//             .insert(correlation_id.clone(), tx);

//         let mut props = BasicProperties::default()
//             .with_correlation_id(correlation_id.clone().into())
//             .with_reply_to(self.reply_queue.clone().into());

//         info!(
//             "Sending RPC request with correlation ID: {} and reply queue: {}",
//             correlation_id, self.reply_queue
//         );

//         if let Some(p) = priority {
//             props = props.with_priority(p);
//         }

//         self.channel
//             .basic_publish(
//                 &self.exchange_name,
//                 "",
//                 BasicPublishOptions::default(),
//                 message.as_bytes(),
//                 props,
//             )
//             .await?
//             .await?;

//         let response = rx
//             .await
//             .map_err(|_| anyhow::anyhow!("RPC response channel closed"));

//         response
//     }
// }
