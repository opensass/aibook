use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user: ObjectId,
    pub content: String,
    #[serde(rename = "bookType")]
    pub book_type: Option<String>,
    #[serde(rename = "mainTopic")]
    pub main_topic: Option<String>,
    pub cover: Option<String>,
    pub completed: bool,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
