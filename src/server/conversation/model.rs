#![allow(non_snake_case)]

use bson::{oid::ObjectId, serde_helpers::chrono_datetime_as_bson_datetime};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Conversation {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user: ObjectId,
    pub book: String,
    pub chapter: Option<String>,
    pub title: String,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono_datetime_as_bson_datetime", rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub conversation: ObjectId,
    pub sender: String,
    pub content: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub timestamp: DateTime<Utc>,
}
