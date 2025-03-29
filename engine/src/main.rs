use engine::{
    configuration::get_configuration,
    messaging::{
        connection::RabbitConnection, consumer_client::ConsumerClient,
        publish_client::PublishClient, rpc_client::RpcClient,
    },
    services::{jackpot::JackpotService, processor::JackpotProcessor},
    telemetry::{get_subscriber, init_subscriber},
};
use lapin::ExchangeKind;
use secrecy::ExposeSecret;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("engine".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let jackpot_service =
        Arc::new(JackpotService::new(&configuration.redis.uri.expose_secret()).await?);

    // Set up RabbitMQ connections
    let gateway_connection = RabbitConnection::new(&configuration.rabbitmq.gateway_url).await?;
    let storage_connection = RabbitConnection::new(&configuration.rabbitmq.storage_url).await?;

    // Set up Storage RPC client
    let storage_rpc_client =
        Arc::new(RpcClient::new(&storage_connection, "storage", ExchangeKind::Direct).await?);

    let publish_client =
        Arc::new(PublishClient::new(&storage_connection, "storage", ExchangeKind::Direct).await?);

    let processor = Arc::new(JackpotProcessor {
        jackpot_service,
        storage_rpc_client,
        publish_client,
    });

    let consumer_client = ConsumerClient::new(
        &gateway_connection,
        "gateway",
        ExchangeKind::Direct,
        "gateway_queue",
        "",
        processor.clone(),
    )
    .await?;

    let gateway_consumer = tokio::spawn(async move { consumer_client.start_consuming().await });

    tokio::select! {
        o = gateway_consumer => report_exit("Gateway Consumer", o),
    }

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
