use actix_web::web::{ServiceConfig, get};
pub mod auth;
mod index;

pub fn endpoints_factory(app: &mut ServiceConfig) {
    app.route("/", get().to(index::index));
    app.configure(auth::auth_endpoints_factory);
}