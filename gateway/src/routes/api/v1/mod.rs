use actix_web::web;

pub mod health;
pub mod wager;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/wager").configure(wager::init));
    cfg.service(web::scope("/health").configure(health::init));
}
