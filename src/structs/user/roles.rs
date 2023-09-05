use serde::Serialize;

#[derive(Serialize)]
pub struct Permission {
    pub id: String,
    pub path: String
}

#[derive(Serialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: Vec<Permission>
}
