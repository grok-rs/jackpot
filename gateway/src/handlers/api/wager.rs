use crate::{
    domain::models::{WagerRequest, WagerResponse},
    messaging::rpc_client::RpcClient,
};
use actix_web::{web, HttpResponse};

// POST / - Creates a wager and sends it to RabbitMQ
pub async fn create_wager(
    rpc_client: web::Data<RpcClient<WagerResponse>>,
    request: web::Json<WagerRequest>,
) -> HttpResponse {
    match rpc_client.call(&request.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().json("Failed to process wager"),
    }
}

pub async fn get_wager() -> HttpResponse {
    HttpResponse::Ok().json(format!("Wager details for ID:"))
}
