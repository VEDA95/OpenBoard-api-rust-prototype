use actix_web::web::{ServiceConfig, get, post, patch, delete};
pub mod user;

pub fn auth_endpoints_factory(app: &mut ServiceConfig) {
    app.route("/auth/users", get().to(user::view::list_users));
}