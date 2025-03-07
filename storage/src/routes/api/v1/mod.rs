use actix_web::web;

pub mod health;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").configure(health::init));
}
