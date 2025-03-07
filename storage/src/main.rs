// use secrecy::ExposeSecret;
// use sqlx::{Executor, postgres::PgPoolOptions};
// use storage::configuration::get_configuration;

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     let config = get_configuration()?;
//     let postgres = &config.postgres;
//     let username = postgres.username.expose_secret();
//     let password = postgres.password.expose_secret();
//     let connection_string = format!(
//         "postgres://{}:{}@{}:{}/{}",
//         username, password, postgres.host, postgres.port, postgres.database_name
//     );

//     let pool = PgPoolOptions::new()
//         .max_connections(5)
//         .connect(&connection_string)
//         .await?;

//     pool.execute("select 1;").await?;

//     Ok(())
// }

use std::{
    fmt::{Debug, Display},
    sync::Arc,
};
use storage::{
    application::Application,
    configuration::get_configuration,
    messaging::connection::RabbitConnection,
    services::{storage::StorageService, storage_processor::TrunsatictionProcessor},
    telemetry::{get_subscriber, init_subscriber},
    worker::start_worker,
};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("engine".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());

    let storage_service = Arc::new(StorageService::new().await?);

    // Set up RabbitMQ connections
    let storage_conn = RabbitConnection::new(&configuration.rabbitmq.uri).await?;

    // Create channels
    let storage_channel = storage_conn.create_channel().await?;

    // Declare the "storage" exchange (direct type, matching storage's configuration)
    storage_channel
        .exchange_declare(
            "storage",
            lapin::ExchangeKind::Direct,
            lapin::options::ExchangeDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    // Declare the queue for Engine to consume from
    storage_channel
        .queue_declare(
            "storage_queue",
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    // Bind the queue to the "storage" exchange with routing key ""
    storage_channel
        .queue_bind(
            "storage_queue",
            "storage",
            "", // Matches storage's publishing routing key
            lapin::options::QueueBindOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await?;

    let processor = Arc::new(TrunsatictionProcessor { storage_service });

    let storage_consumer = tokio::spawn(start_worker(storage_channel, processor));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = storage_consumer => report_exit("Engine Consumer", o),
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
