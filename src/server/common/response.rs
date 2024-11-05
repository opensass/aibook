use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SuccessResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
