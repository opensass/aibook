use crate::server::conversation::model::Conversation;
use crate::server::conversation::model::Message;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageResponse {
    pub status: String,
    pub data: Message,
}
