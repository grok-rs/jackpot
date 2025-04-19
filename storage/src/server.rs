use crate::configuration::ApplicationSettings;
use crate::messaging::connection::RabbitConnection;
use anyhow::Result;
use sqlx::PgPool;
use std::future::Future;
use std::sync::Arc;
use tracing::{debug, info, warn};
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

pub async fn start_server(
    app_config: ApplicationSettings,
    storage_connection: Arc<RabbitConnection>,
    pg_pool: Arc<PgPool>, // Add pool as a parameter
) -> Result<impl Future<Output = ()>> {
    info!("Starting server on {}:{}", app_config.host, app_config.port);

    let health_route = warp::path("health").and_then(move || {
        let storage_connection = storage_connection.clone();
        let pg_pool = pg_pool.clone();
        health_check(storage_connection, pg_pool) // Pass both to health_check
    });

    Ok(warp::serve(health_route).run((app_config.host, app_config.port)))
}

async fn health_check(
    storage_connection: Arc<RabbitConnection>,
    pg_pool: Arc<PgPool>,
) -> Result<impl Reply, Rejection> {
    debug!("Performing health check");

    // Check RabbitMQ connection
    let storage_ok = storage_connection.is_connected();
    debug!("RabbitMQ connection: {}", storage_ok);

    let pg_ok = sqlx::query("SELECT 1").execute(&*pg_pool).await.is_ok();
    debug!("PostgreSQL connection: {}", pg_ok);

    if storage_ok && pg_ok {
        debug!("Health check: healthy");
        Ok(warp::reply::with_status(
            "healthy".to_string(),
            StatusCode::OK,
        ))
    } else {
        warn!(
            "Health check: unhealthy - RabbitMQ: {}, PostgreSQL: {}",
            storage_ok, pg_ok
        );
        let error_msg = format!("Unhealthy: RabbitMQ={}, PostgreSQL={}", storage_ok, pg_ok);
        Ok(warp::reply::with_status(
            error_msg,
            StatusCode::SERVICE_UNAVAILABLE,
        ))
    }
}
