use actix_web::web::{ServiceConfig, get};
pub mod structs;
mod index;
mod auth;

pub fn endpoints_factory(app: &mut ServiceConfig) {
    app.route("/", get().to(index::index));
}