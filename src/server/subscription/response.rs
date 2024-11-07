use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionDetailResponse {
    pub session: String,
    pub method: String,
}
