use actix_web::web::{ServiceConfig, get};
mod index;
mod auth;

pub fn endpoints_factory(app: &mut ServiceConfig) {
    app.route("/", get().to(index::index));
}