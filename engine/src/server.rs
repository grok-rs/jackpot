use crate::configuration::ApplicationSettings;
use crate::messaging::connection::RabbitConnection;
use anyhow::Result;
use std::future::Future;
use std::sync::Arc;
use tracing::{debug, info, warn};
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

pub async fn start_server(
    app_config: ApplicationSettings,
    gateway_connection: Arc<RabbitConnection>,
    storage_connection: Arc<RabbitConnection>,
) -> Result<impl Future<Output = ()>> {
    info!("Starting server on {}:{}", app_config.host, app_config.port);
    let health_route = warp::path("health").and_then(move || {
        let gateway_connection = gateway_connection.clone();
        let storage_connection = storage_connection.clone();
        let connections = (gateway_connection, storage_connection);
        health_check(connections)
    });

    Ok(warp::serve(health_route).run((app_config.host, app_config.port)))
}

async fn health_check(
    (gateway_connection, storage_connection): (Arc<RabbitConnection>, Arc<RabbitConnection>),
) -> Result<impl Reply, Rejection> {
    debug!("Performing health check");
    let gateway_ok = gateway_connection.is_connected();
    let storage_ok = storage_connection.is_connected();
    debug!(
        "Gateway connection: {}, Storage connection: {}",
        gateway_ok, storage_ok
    );

    if gateway_ok && storage_ok {
        debug!("Health check: healthy");
        Ok(warp::reply::with_status(
            "healthy".to_string(),
            StatusCode::OK,
        ))
    } else {
        warn!(
            "Health check: unhealthy - gateway: {}, storage: {}",
            gateway_ok, storage_ok
        );
        let error_msg = format!(
            "Unhealthy: gateway_connection={}, storage_connection={}",
            gateway_ok, storage_ok
        );
        Ok(warp::reply::with_status(
            error_msg,
            StatusCode::SERVICE_UNAVAILABLE,
        ))
    }
}
