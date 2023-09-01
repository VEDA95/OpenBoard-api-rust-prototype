use actix_web::web::{ServiceConfig, get};
mod index;

pub fn endpoints_factory(app: &mut ServiceConfig) {
    app.route("/", get().to(index::index));
}