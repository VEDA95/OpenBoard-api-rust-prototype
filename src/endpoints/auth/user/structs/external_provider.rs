use serde::Serialize;

#[derive(Serialize)]
pub struct UserExternalProvider {
    id: String,
    name: String,
    default_login_method: bool
}