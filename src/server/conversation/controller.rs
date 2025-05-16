#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::server::auth::controller::auth;
use crate::server::book::model::Book;
use crate::server::book::model::Chapter;
use crate::server::common::response::SuccessResponse;
use crate::server::conversation::model::Conversation;
use crate::server::conversation::model::Message;
use crate::server::conversation::request::CreateConversationRequest;
use crate::server::conversation::request::GetConversationsRequest;
use crate::server::conversation::request::GetMessagesRequest;
use crate::server::conversation::request::SendQueryRequest;
use crate::server::conversation::response::ConversationResponse;
use crate::server::conversation::response::ConversationsListResponse;
use crate::server::conversation::response::MessageResponse;
use crate::server::conversation::response::MessagesListResponse;
use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::TryStreamExt;
use std::env;
#[cfg(feature = "server")]
use {
    crate::ai::get_ai, crate::db::get_client, gems::chat::ChatBuilder, gems::messages::Content,
    gems::messages::Message as GMessage, gems::models::Model, gems::traits::CTrait,
};

#[server]
pub async fn create_conversation(
    req: CreateConversationRequest,
) -> Result<ConversationResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let conversation_collection = db.collection::<Conversation>("conversations");

    let conversation = Conversation {
        id: ObjectId::new(),
        user: user.id,
        book: req.book_id,
        chapter: None,
        title: req.title,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    conversation_collection
        .insert_one(conversation.clone())
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(ConversationResponse {
        status: "success".to_string(),
        data: conversation,
    })
}

#[server]
pub async fn get_conversations(
    req: GetConversationsRequest,
) -> Result<ConversationsListResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let conversation_collection = db.collection::<Conversation>("conversations");

    let filter = doc! {"user": user.id, "book": req.book_id};
    let cursor = conversation_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    let conversations: Vec<Conversation> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(ConversationsListResponse {
        status: "success".to_string(),
        data: conversations,
    })
}

#[server]
pub async fn save_message_to_db(message: Message) -> Result<(), ServerFnError> {
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");

    messages_collection
        .insert_one(message)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    Ok(())
}

#[server]
pub async fn get_messages(req: GetMessagesRequest) -> Result<MessagesListResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");

    let filter = doc! {"conversation": req.conversation_id};
    let cursor = messages_collection
        .find(filter)
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;
    let messages: Vec<Message> = cursor
        .try_collect()
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(MessagesListResponse {
        status: "success".to_string(),
        data: messages,
    })
}

#[server]
pub async fn send_query_to_gemini(req: SendQueryRequest) -> Result<MessageResponse, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let messages_collection = db.collection::<Message>("messages");
    let book_collection = db.collection::<Book>("books");
    let chapters_collection = db.collection::<Chapter>("chapters");

    let client = get_ai(req.model.to_string()).await.lock().await;

    let book_id =
        ObjectId::parse_str(&req.book).map_err(|_| ServerFnError::new("Invalid book ID"))?;

    let book = book_collection
        .find_one(doc! { "_id": book_id, "user": user.id })
        .await?
        .ok_or(ServerFnError::new("Book not found"))?;

    let chapter_id =
        ObjectId::parse_str(&req.chapter).map_err(|_| ServerFnError::new("Invalid chapter ID"))?;

    let chapter = chapters_collection
        .find_one(doc! { "_id": chapter_id })
        .await?
        .ok_or(ServerFnError::new("Chapter not found"))?;

    let system_prompt = format!(
        "
        **System Prompt (SP):** You are a knowledgeable assistant specializing in providing in-depth responses based on specific book chapters. You understand the structure, themes, and content of books, and you answer questions with context and precision.
        Generate your response as HTML-formatted response with examples, links and images, based on the query: '{user_query}'. \
        Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
        <h2> for chapter titles, <h3> for subheadings, and <p> for paragraphs. \
        Include well-organized, readable content that aligns with the book's title {book_title}, ensuring each section is \
        clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
        if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
        cover all relevant subtopics in depth to create an engaging reading experience.

        **Context Information:**
        - Book Title: '{book_title}'
        - Chapter: '{chapter}'
    
        **User Query:** The user has a question related to the book chapter provided. Use the context of the book and chapter to respond accurately and engage thoughtfully.
    
        **Prompt (P):** Answer the user's question in detail, focusing on information specific to the chapter content '{chapter}' from the book '{book_title}'. 
        - Explain complex concepts in an accessible way if the user’s query requires it.
        - Where applicable, relate your answer back to key themes and ideas presented in this chapter.
        - If the chapter has distinct characters, events, or themes, draw on these to enhance your response.
        - Provide direct and actionable information if the question is specific, or a comprehensive overview if the question is broad.
    
        **Expected Format (EF):**
        - Begin with a brief introduction if the question pertains to a major theme or character in the chapter.
        - Answer in a clear, step-by-step, or structured format when applicable.
        - For complex queries, summarize the response in the last sentence to ensure clarity for the user.
    
        Make sure to always return back with html formmatted text and not empty response. If the user asks to translate a chapter, always respond with the corresponding translation and never reject the request.

        **Roleplay (RP):** Act as a well-read, insightful assistant dedicated to enhancing the reader’s understanding of the material in this book chapter. Aim to be both informative and engaging in your response.
    
        **User Query:** '{user_query}'
        ",
        book_title = book.title,
        chapter = chapter.html,
        user_query = req.query
    );

    let parameters = ChatBuilder::default()
        .model(Model::Flash20)
        .messages(vec![GMessage::User {
            content: Content::Text(system_prompt.to_string()),
            name: None,
        }])
        .build()?;

    let content = client
        .chat()
        .generate(parameters)
        .await
        .map_err(ServerFnError::new)?
        .trim_start_matches("```html")
        .trim_end_matches("```")
        .trim()
        .to_string();

    let response_message = Message {
        id: ObjectId::new(),
        conversation: req.conversation_id,
        sender: "gemini".to_string(),
        content: content,
        timestamp: Utc::now(),
    };

    messages_collection
        .insert_one(response_message.clone())
        .await
        .map_err(|e| ServerFnError::new(&e.to_string()))?;

    Ok(MessageResponse {
        status: "success".to_string(),
        data: response_message,
    })
}
