use std::net::TcpListener;

use crate::{
    configuration::{Config, RabbitMqConfig},
    domain::models::WagerResponse,
    messaging::{connection::RabbitConnection, rpc_client::RpcClient},
    routes,
};
use actix_web::{App, HttpServer, web::Data};

use actix_web::dev::Server;
use lapin::ExchangeKind;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Config) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            configuration.application.base_url,
            configuration.rabbitmq,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

async fn run(
    listener: TcpListener,
    base_url: String,
    rabbitmq_config: RabbitMqConfig,
) -> Result<Server, anyhow::Error> {
    let base_url = Data::new(ApplicationBaseUrl(base_url));

    let conn = RabbitConnection::new(&rabbitmq_config.uri).await?;

    let rpc_client = RpcClient::<WagerResponse>::new(
        &conn,
        &rabbitmq_config.exchange_name,
        ExchangeKind::Direct,
    )
    .await?;
    let rpc_client = Data::new(rpc_client);

    let server = HttpServer::new(move || {
        App::new()
            .configure(routes::init)
            .app_data(base_url.clone())
            .app_data(rpc_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
