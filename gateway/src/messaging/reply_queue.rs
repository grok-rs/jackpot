use std::sync::Arc;

use anyhow::Context;
use lapin::{options::QueueDeclareOptions, types::FieldTable, Channel};

pub struct ReplyQueue {
    channel: Arc<Channel>,
    name: String,
}

impl ReplyQueue {
    pub async fn new(channel: Arc<Channel>) -> anyhow::Result<Self> {
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

        Ok(Self {
            channel,
            name: queue.name().as_str().to_string(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
