use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditUserSchema {
    pub token: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub old_password: Option<String>,
    pub new_password: Option<String>,
    pub confirm_password: Option<String>,
}
