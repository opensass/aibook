#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::ai::get_ai;
#[cfg(feature = "server")]
use crate::db::get_client;
use crate::server::auth::controller::auth;
use crate::server::book::model::Book;
use crate::server::book::request::CompleteBookRequest;
use crate::server::book::request::GenerateBookRequest;
use crate::server::book::request::GetBookForUserRequest;
use crate::server::book::request::GetBooksForUserRequest;
use crate::server::book::request::StoreBookRequest;
use crate::server::book::request::UpdateBookContentRequest;
use crate::server::book::response::BookResponse;
use crate::server::common::response::SuccessResponse;
use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::TryStreamExt;

#[server]
pub async fn store_book(
    req: StoreBookRequest,
) -> Result<SuccessResponse<BookResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let book_collection = db.collection::<Book>("books");

    let photo_url = match req.main_topic {
        Some(ref topic) => fetch_cover(topic).await?,
        None => None,
    };

    let new_book = Book {
        id: ObjectId::new(),
        user: user.id,
        content: req.content,
        book_type: req.book_type,
        main_topic: req.main_topic,
        cover: photo_url,
        completed: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    book_collection.insert_one(new_book.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: BookResponse { id: new_book.id },
    })
}

async fn fetch_cover(topic: &str) -> Result<Option<String>, ServerFnError> {
    Ok(Some("photo_url".into()))
}

#[server]
pub async fn update_book_content(
    req: UpdateBookContentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let book_collection = db.collection::<Book>("books");

    book_collection
        .update_one(
            doc! { "_id": req.book_id },
            doc! { "$set": { "content": req.new_content, "updatedAt": Utc::now() } },
        )
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: "Book updated successfully".into(),
    })
}

#[server]
pub async fn complete_book(
    req: CompleteBookRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let book_collection = db.collection::<Book>("books");

    book_collection
        .update_one(
            doc! { "_id": req.book_id },
            doc! { "$set": { "completed": true, "updatedAt": Utc::now() } },
        )
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: "Book marked as completed".into(),
    })
}

#[server]
pub async fn get_books_for_user(
    req: GetBooksForUserRequest,
) -> Result<SuccessResponse<Vec<Book>>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let book_collection = db.collection::<Book>("books");

    let books = book_collection
        .find(doc! { "user": user.id })
        .await?
        .try_collect()
        .await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: books,
    })
}

#[server]
pub async fn get_book_for_user(
    req: GetBookForUserRequest,
) -> Result<SuccessResponse<Book>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let book_collection = db.collection::<Book>("books");

    let book_id =
        ObjectId::parse_str(&req.book_id).map_err(|_| ServerFnError::new("Invalid book ID"))?;

    let book = book_collection
        .find_one(doc! { "_id": book_id, "user": user.id })
        .await?
        .ok_or(ServerFnError::new("Book not found"))?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: book,
    })
}

#[server]
pub async fn gemini_call(
    req: GenerateBookRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let mut client = get_ai(req.model.to_string()).await.lock().await;

    let mut prompt = format!(
        "Generate a comprehensive book titled '{}' with the subtitle '{}'. \
         The book should be written in {} and consist of {} chapters, each \
         covering multiple subtopics up to a total of {}. Ensure each chapter \
         provides a thorough exploration of topics relevant to '{}' and maintain a \
         maximum length of {} words. \
         Structure the book with sections and subtopics that align with the title and subtitle, \
         and ensure consistency, quality, and depth throughout. Avoid filler content and focus \
         on detailed and engaging explanations, while maintaining clarity and readability.",
        req.title,
        req.subtitle,
        req.language,
        req.chapters,
        req.subtopics,
        req.title,
        req.max_length,
    );

    let mut result = "".to_string();
    // get book structure
    match client.generate_content(&prompt).await {
        Ok(response) => {
            result = response;
        }
        Err(error) => {}
    }
    // generate all chapters content
    prompt = format!(
        "Generate a comprehensive HTML-formatted book based on the outline: {}. \
        Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
        <h2> for chapter titles, <h3> for subheadings, and <p> for paragraphs. \
        Include well-organized, readable content that aligns with the book's outline, ensuring each section is \
        clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
        if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
        cover all relevant subtopics in depth to create an engaging reading experience.",
        result,
    );

    match client.generate_content(&prompt).await {
        Ok(response) => Ok(SuccessResponse {
            status: "success".into(),
            data: response.into(),
        }),
        Err(error) => Err(ServerFnError::new(error)),
    }
}
