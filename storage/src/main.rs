use std::{
    fmt::{Debug, Display},
    sync::Arc,
};
use storage::{
    configuration::get_configuration,
    db::wager_repository::PostgresWagerRepository,
    messaging::{connection::RabbitConnection, consumer_client::ConsumerClient},
    services::{storage::StorageService, storage_processor::TrunsatictionProcessor},
    telemetry::{get_subscriber, init_subscriber},
};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("storage".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let pool = sqlx::PgPool::connect(&configuration.postgres.build_url()).await?;
    let pool = Arc::new(pool);
    let wager_repository = PostgresWagerRepository::new(pool.clone());

    let storage_service = Arc::new(StorageService::new(wager_repository).await?);

    // Set up RabbitMQ connection
    let storage_conn = RabbitConnection::new(&configuration.rabbitmq.uri).await?;

    let processor = Arc::new(TrunsatictionProcessor { storage_service });

    // Set up ConsumerClient
    let consumer_client = ConsumerClient::new(
        &storage_conn,
        "storage",
        lapin::ExchangeKind::Direct,
        "storage_queue",
        "",
        processor,
    )
    .await?;

    let storage_consumer = tokio::spawn(async move { consumer_client.start_consuming().await });

    tokio::select! {
        o = storage_consumer => report_exit("Storage Consumer", o),
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
