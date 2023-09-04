use serde::Serialize;

#[derive(Serialize)]
pub struct Permission {
    id: String,
    path: String
}

#[derive(Serialize)]
pub struct Role {
    id: String,
    name: String,
    permissions: Vec<Permission>
}
