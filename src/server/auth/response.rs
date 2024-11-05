use crate::server::auth::model::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterUserSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserResponse {
    pub user: User,
}
