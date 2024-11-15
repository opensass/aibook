use crate::server::conversation::model::Conversation;
use crate::server::conversation::model::Message;
use bytes::Bytes;
use futures_util::stream;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationResponse {
    pub status: String,
    pub data: Conversation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationsListResponse {
    pub status: String,
    pub data: Vec<Conversation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagesListResponse {
    pub status: String,
    pub data: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub status: String,

    #[serde(skip)]
    pub data: Option<Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>>,
}

impl Default for MessageResponse {
    fn default() -> Self {
        Self {
            status: "success".to_string(),
            data: Some(Box::pin(stream::empty())),
        }
    }
}
