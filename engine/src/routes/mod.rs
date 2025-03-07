use actix_web::web;

pub mod api;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(api::init));
}
