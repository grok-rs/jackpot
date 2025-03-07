use crate::handlers::api::wager::{create_wager, get_wager};
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::post().to(create_wager))
        .route("/{id}", web::get().to(get_wager));
}
