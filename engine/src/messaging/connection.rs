use anyhow::Context;
use lapin::{Channel, Connection, ConnectionProperties};
use secrecy::{ExposeSecret, SecretString};

pub struct RabbitConnection {
    connection: Connection,
}

impl RabbitConnection {
    pub async fn new(rabbitmq_url: &SecretString) -> anyhow::Result<Self> {
        let connection = Connection::connect(
            rabbitmq_url.expose_secret(),
            ConnectionProperties::default(),
        )
        .await
        .context("Failed to connect to RabbitMQ")?;
        Ok(Self { connection })
    }

    pub async fn create_channel(&self) -> anyhow::Result<Channel> {
        self.connection
            .create_channel()
            .await
            .context("Failed to create RabbitMQ channel")
    }
}
