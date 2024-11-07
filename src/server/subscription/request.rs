use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionDetailRequest {
    pub user_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateSubscriptionRequest {
    pub subscription_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StripePaymentRequest {
    pub plan_id: String,
}
