use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Permission {
    pub id: String,
    pub path: String
}

#[derive(Serialize, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: Vec<Permission>
}

impl Role {
    pub fn new(id: String, name: String) -> Role {
        return Role {
            id: id,
            name: name,
            permissions: Vec::new()
        };
    }

    pub fn add_permission(&mut self, permission: Permission) -> () {
        self.permissions.push(permission);
    }
}
