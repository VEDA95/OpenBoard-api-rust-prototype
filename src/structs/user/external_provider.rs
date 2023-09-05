use serde::Serialize;

#[derive(Serialize)]
pub struct UserExternalProvider {
    pub id: String,
    pub name: String,
    pub default_login_method: bool
}