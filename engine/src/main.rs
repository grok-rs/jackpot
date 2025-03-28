use engine::{
    application::Application,
    configuration::get_configuration,
    messaging::{connection::RabbitConnection, rpc_client::RpcClient},
    services::{jackpot::JackpotService, processor::JackpotProcessor},
    telemetry::{get_subscriber, init_subscriber},
    worker::start_worker,
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
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());

    let jackpot_service =
        Arc::new(JackpotService::new(&configuration.redis.uri.expose_secret()).await?);

    // Set up RabbitMQ connections
    let gateway_conn = RabbitConnection::new(&configuration.rabbitmq.gateway_url).await?;
    let storage_conn = RabbitConnection::new(&configuration.rabbitmq.storage_url).await?;

    // Create channels
    let gateway_channel = gateway_conn.create_channel().await?;
    let storage_channel = Arc::new(storage_conn.create_channel().await?);

    // Declare the "gateway" exchange (direct type, matching Gateway's configuration)
    gateway_channel
        .exchange_declare(
            "gateway",
            lapin::ExchangeKind::Direct,
            lapin::options::ExchangeDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    // Declare the queue for Engine to consume from
    gateway_channel
        .queue_declare(
            "gateway_queue",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    // Bind the queue to the "gateway" exchange with routing key ""
    gateway_channel
        .queue_bind(
            "gateway_queue",
            "gateway",
            "", // Matches Gateway's publishing routing key
            lapin::options::QueueBindOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    // Declare the "storage_exchange" for communicating with Storage
    storage_channel
        .exchange_declare(
            "storage",
            lapin::ExchangeKind::Direct,
            lapin::options::ExchangeDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    // Set up Storage RPC client
    //
    //
    let storage_rpc_client =
        Arc::new(RpcClient::new(&storage_conn, "storage", ExchangeKind::Direct).await?);

    let processor = Arc::new(JackpotProcessor {
        jackpot_service,
        storage_rpc_client,
        storage_channel,
    });

    let gateway_consumer = tokio::spawn(start_worker(gateway_channel, processor));

    tokio::select! {
        o = application_task => report_exit("API", o),
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
