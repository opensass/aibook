use crate::server::book::model::Book;
use crate::server::book::model::Chapter;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookResponse {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateBookOutlineResponse {
    pub chapters: Vec<Chapter>,
    pub book: Book,
}
