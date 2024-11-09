use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoreBookRequest {
    pub token: String,
    pub title: String,
    pub subtitle: String,
    pub content: String,
    pub book_type: Option<String>,
    pub main_topic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateBookContentRequest {
    pub token: String,
    pub book_id: String,
    pub new_content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompleteBookRequest {
    pub book_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetBooksForUserRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateBookRequest {
    pub title: String,
    pub subtitle: String,
    pub token: String,
    pub model: String,
    pub subtopics: u64,
    pub chapters: u64,
    pub language: String,
    pub max_length: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateChapterContentRequest {
    pub chapter_title: String,
    pub chapter_id: ObjectId,
    pub book_title: String,
    pub main_topic: String,
    pub language: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetBookForUserRequest {
    pub token: String,
    pub book_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIRequest {
    pub token: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChaptersContentRequest {
    pub book_id: String,
}
