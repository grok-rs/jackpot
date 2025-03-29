use crate::{
    domain::models::{WagerRequest, WagerResponse},
    messaging::rpc_client::RpcClient,
};
use actix_web::{HttpResponse, web};
use uuid::Uuid;

// POST / - Creates a wager and sends it to RabbitMQ
pub async fn create_wager(
    rpc_client: web::Data<RpcClient<WagerResponse>>,
    request: web::Json<WagerRequest>,
) -> HttpResponse {
    let mut request = request.into_inner();

    if request.id.is_none() {
        request.id = Some(Uuid::new_v4());
    }

    match rpc_client.call(&request).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().json("Failed to process wager"),
    }
}
