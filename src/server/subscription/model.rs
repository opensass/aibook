use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Subscription {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user: ObjectId,
    pub sub: String,
    #[serde(rename = "subId")]
    pub sub_id: String,
    pub plan: String,
    pub method: String,
    pub active: bool,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
