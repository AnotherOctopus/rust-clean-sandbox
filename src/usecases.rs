use crate::entities;

pub fn create_user(name: String, pass: String) -> entities::User {
    entities::User::new(name.to_string(), pass.to_string())
}

pub fn create_admin(name: String, pass: String) -> entities::Admin {
    entities::Admin::new(name.to_string(), pass.to_string())
}
