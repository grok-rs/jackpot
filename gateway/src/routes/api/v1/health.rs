use crate::handlers::api::health::health_check;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(health_check));
}
