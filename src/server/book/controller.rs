#![allow(unused)]
#![allow(dead_code)]

use bson::doc;
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::server::auth::controller::auth;
use crate::server::book::model::Book;
use crate::server::book::model::Chapter;
use crate::server::book::request::AIRequest;
use crate::server::book::request::CompleteBookRequest;
use crate::server::book::request::GenerateBookRequest;
use crate::server::book::request::GenerateChapterContentRequest;
use crate::server::book::request::GetBookForUserRequest;
use crate::server::book::request::GetBooksForUserRequest;
use crate::server::book::request::GetChaptersContentRequest;
use crate::server::book::request::StoreBookRequest;
use crate::server::book::request::UpdateBookContentRequest;
use crate::server::book::response::BookResponse;
use crate::server::book::response::GenerateBookOutlineResponse;
use crate::server::book::response::{
    AIUsageStats, AnalyticsData, EngagementStats, PredictiveStats,
};
use crate::server::common::response::SuccessResponse;
use std::env;

use bson::oid::ObjectId;
use chrono::prelude::*;
use futures_util::StreamExt;
use futures_util::TryStreamExt;
use regex::Regex;
#[cfg(feature = "server")]
use {
    crate::ai::get_ai,
    crate::db::get_client,
    crate::unsplash::get_unsplash_client,
    http_api_isahc_client::{Client as _, IsahcClient},
    rand::thread_rng,
    rand::Rng,
    unsplash_api::endpoints::common::EndpointRet,
    unsplash_api::endpoints::search_photos::SearchPhotos,
    unsplash_api::endpoints::search_photos::SearchPhotosResponseBodyOkJson,
    unsplash_api::objects::pagination::Pagination,
    unsplash_api::objects::rate_limiting::RateLimiting,
};

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
        Some(ref topic) => fetch_cover(topic.to_string()).await?,
        None => None,
    };

    let new_book = Book {
        id: ObjectId::new(),
        user: user.id,
        title: req.title,
        subtitle: Some(req.subtitle),
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

#[server]
pub async fn fetch_cover(topic: String) -> Result<Option<String>, ServerFnError> {
    let client = get_unsplash_client().await.lock().await;

    let search_photos = SearchPhotos::new(
        &env::var("UNSPLASH_API_KEY").expect("UNSPLASH_API_KEY must be set."),
        topic,
    );

    let response: EndpointRet<(SearchPhotosResponseBodyOkJson, Pagination, RateLimiting)> =
        client.respond_endpoint(&search_photos).await?;

    let mut extracted_data = Vec::new();

    if let EndpointRet::Ok((ok_json, _pagination, _rate_limiting)) = response {
        for photo in ok_json.results {
            let image_url = photo.urls.regular.to_string();

            extracted_data.push(image_url);
        }
    } else {
        tracing::error!("Unexpected response type");
    }

    if extracted_data.is_empty() {
        return Ok(None);
    }

    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..extracted_data.len());
    Ok(Some(extracted_data[random_index].clone()))
}

#[server]
pub async fn update_book_content(
    req: UpdateBookContentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let book_collection = db.collection::<Book>("books");

    let book_id =
        ObjectId::parse_str(&req.book_id).map_err(|_| ServerFnError::new("Invalid book ID"))?;

    book_collection
        .update_one(
            doc! { "_id": book_id },
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
pub async fn generate_book_outline(
    req: GenerateBookRequest,
) -> Result<SuccessResponse<GenerateBookOutlineResponse>, ServerFnError> {
    let user = auth(req.token)
        .await
        .map_err(|_| ServerFnError::new("Not Authenticated"))?;

    let mut client = get_ai(req.model.to_string()).await.lock().await;

    let outline_prompt = format!(
        "
        **System Prompt (SP):** You are an expert in book creation, generating a structured outline.

        **Prompt (P):** Generate an outline for a book titled '{title}', with subtitle '{subtitle}'. Main topic is '{title}' in {language}. The book should contain {chapters} chapters covering {subtopics} subtopics. Provide an estimated duration for each chapter.

        **Expected Format (EF):**
        ### Chapter [number]: [Chapter Title]
        **Estimated Duration:** [Duration] minutes

        **Roleplay (RP):** As a book editor, create an engaging outline.
        ",
        title = req.title,
        subtitle = req.subtitle,
        chapters = req.chapters,
        subtopics = req.subtopics,
        language = req.language,
    );

    let outline = client
        .generate_content(&outline_prompt)
        .await
        .map_err(ServerFnError::new)?;

    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let book_collection = db.collection::<Book>("books");

    let photo_url = fetch_cover(req.title.clone()).await?;

    let book = Book {
        id: ObjectId::new(),
        user: user.id,
        title: req.title.clone(),
        subtitle: Some(req.subtitle.clone()),
        book_type: Some(req.title.clone()),
        main_topic: Some(req.title.clone()),
        completed: false,
        cover: photo_url,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    book_collection.insert_one(book.clone()).await?;

    let chapters = parse_outline(outline.clone(), book.id, req.language)?;

    let chapters_collection = db.collection::<Chapter>("chapters");
    chapters_collection.insert_many(chapters.clone()).await?;

    Ok(SuccessResponse {
        status: "success".into(),
        data: GenerateBookOutlineResponse {
            book: book.clone(),
            chapters: chapters.clone(),
        },
    })
}

fn parse_outline(
    outline: String,
    book_id: ObjectId,
    language: String,
) -> Result<Vec<Chapter>, ServerFnError> {
    let mut chapters = Vec::new();

    let re =
        Regex::new(r"### Chapter (\d+):\s*(.*?)\s*\n\*\*Estimated Duration:\*\*\s*(\d+)\s*minutes")
            .unwrap();

    let mut current_position = 0;
    while let Some(caps) = re.captures(&outline[current_position..]) {
        let title = &caps[2];
        let estimated_duration = caps[3].parse().unwrap_or(0);

        let next_chapter_pos = outline[current_position..]
            .find("### Chapter ")
            .unwrap_or(outline.len());

        let chapter_content = &outline[current_position..current_position + next_chapter_pos];

        let bullet_points_re = Regex::new(r"\* .+").unwrap();
        let bullet_points = bullet_points_re
            .find_iter(chapter_content)
            .map(|mat| mat.as_str())
            .collect::<Vec<&str>>()
            .join("\n");

        chapters.push(Chapter {
            id: ObjectId::new(),
            book_id,
            title: title.to_string(),
            estimated_duration,
            markdown: bullet_points.trim().to_string(),
            html: String::new(),
            completed: false,
            language: language.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        current_position += next_chapter_pos + chapter_content.len();
    }

    Ok(chapters)
}

#[server]
pub async fn generate_chapter_content(
    req: GenerateChapterContentRequest,
) -> Result<SuccessResponse<String>, ServerFnError> {
    let mut client = get_ai(req.model.to_string()).await.lock().await;

    let content_prompt = format!(
        "
        **System Prompt (SP):** You are writing detailed content for a book chapter.

        **Prompt (P):** Write content for chapter '{chapter_title}' of the book '{book_title}' on the main topics '{main_topic}' in {language}. Ensure clarity, detailed explanations, and structured markdown.

        **Expected Format (EF):**
        - detailed markdown format for this chapter.

        **Roleplay (RP):** Provide as much educational content as possible.
        ",
        chapter_title = req.chapter_title,
        book_title = req.book_title,
        main_topic = req.main_topic,
        language = req.language,
    );

    let markdown = client
        .generate_content(&content_prompt)
        .await
        .map_err(ServerFnError::new)?;

    let content_prompt = format!(
        "Generate a comprehensive HTML-formatted book chapter with examples, links and images, based on the outline: '{}' in {language}. \
        Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
        <h2> for chapter titles, <h3> for subheadings, and <p> for paragraphs. \
        Include well-organized, readable content that aligns with the book's outline, ensuring each section is \
        clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
        if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
        cover all relevant subtopics in depth to create an engaging reading experience. \
        Make sure to always return back with html formmatted text and not empty response.
        ",
        markdown.clone(),
        language = req.language,
    );
    let html = client
        .generate_content(&content_prompt)
        .await
        .map_err(ServerFnError::new)?
        .trim_start_matches("```html")
        .trim_end_matches("```")
        .trim()
        .to_string();

    update_chapter_content(req.chapter_id, markdown.clone(), html.clone()).await?;
    Ok(SuccessResponse {
        status: "success".into(),
        data: html,
    })
}

#[server]
pub async fn get_chapters_for_book(
    req: GetChaptersContentRequest,
) -> Result<SuccessResponse<Vec<Chapter>>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let chapter_collection = db.collection::<Chapter>("chapters");

    let book_object_id =
        ObjectId::parse_str(&req.book_id).map_err(|_| ServerFnError::new("Invalid book ID"))?;

    let mut chapters = chapter_collection
        .find(doc! { "book_id": book_object_id })
        .await?
        .try_collect::<Vec<Chapter>>()
        .await?;

    for chapter in chapters.iter_mut() {
        if chapter.html.is_empty() {
            let markdown_content = chapter.markdown.clone();

            let content_prompt = format!(
                "Generate a comprehensive HTML-formatted book chapter with examples, links and images, based on the outline: '{}' in {language}. \
                Each section should be structured with appropriate HTML tags, including <h1> for the main title, \
                <h2> for chapter titles, <h3> for subheadings, and <p> for paragraphs. \
                Include well-organized, readable content that aligns with the book's outline, ensuring each section is \
                clear and logically flows from one to the next. Avoid markdown format entirely, and provide inline HTML styling \
                if necessary to enhance readability. The HTML content should be well-formatted, semantically correct, and \
                cover all relevant subtopics in depth to create an engaging reading experience. \
                Make sure to always return back with html formmatted text and not empty response.",
                markdown_content,
                language = chapter.language,
            );

            let mut ai_client = get_ai("gemini-pro".to_string()).await.lock().await;
            let html_content = ai_client
                .generate_content(&content_prompt)
                .await
                .map_err(ServerFnError::new)?
                .trim_start_matches("```html")
                .trim_end_matches("```")
                .trim()
                .to_string();

            chapter_collection
                .update_one(
                    doc! { "_id": chapter.id },
                    doc! { "$set": { "html": html_content.clone(), "updatedAt": Utc::now() } },
                )
                .await?;

            chapter.html = html_content;
        }
    }

    Ok(SuccessResponse {
        status: "success".into(),
        data: chapters,
    })
}

#[server]
async fn update_chapter_content(
    chapter_id: ObjectId,
    markdown_content: String,
    html_content: String,
) -> Result<(), ServerFnError> {
    let db_client = get_client().await;
    let db = db_client
        .database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));
    let chapters_collection = db.collection::<Chapter>("chapters");

    let update_doc = doc! {
        "$set": {
            "markdown": markdown_content,
            "html": html_content,
            "completed": true,
            "updatedAt": Utc::now(),
        }
    };

    chapters_collection
        .update_one(doc! { "_id": chapter_id }, update_doc)
        .await
        .map_err(|_| ServerFnError::new("Database error"))?;

    Ok(())
}

#[server]
pub async fn summarize_text(req: AIRequest) -> Result<SuccessResponse<String>, ServerFnError> {
    let mut client = get_ai("gemini-pro".to_string()).await.lock().await;
    let prompt = format!("Summarize the following text: '{}'", req.text);

    match client.generate_content(&prompt).await {
        Ok(summary) => Ok(SuccessResponse {
            status: "success".into(),
            data: summary.into(),
        }),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

#[server]
pub async fn regenerate_text(req: AIRequest) -> Result<SuccessResponse<String>, ServerFnError> {
    let mut client = get_ai("gemini-pro".to_string()).await.lock().await;
    let prompt = format!("Rephrase the following text: '{}'", req.text);

    match client.generate_content(&prompt).await {
        Ok(rephrased) => Ok(SuccessResponse {
            status: "success".into(),
            data: rephrased.into(),
        }),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

#[server]
pub async fn extend_text(req: AIRequest) -> Result<SuccessResponse<String>, ServerFnError> {
    let mut client = get_ai("gemini-pro".to_string()).await.lock().await;
    let prompt = format!(
        "Expand on the following text with additional details: '{}'",
        req.text
    );

    match client.generate_content(&prompt).await {
        Ok(extended) => Ok(SuccessResponse {
            status: "success".into(),
            data: extended.into(),
        }),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

#[server]
pub async fn fetch_analytics_data() -> Result<SuccessResponse<AnalyticsData>, ServerFnError> {
    let client = get_client().await;
    let db =
        client.database(&std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set."));

    let books_collection = db.collection::<Book>("books");
    let chapters_collection = db.collection::<Chapter>("chapters");

    // Engagement Metrics
    let total_books = books_collection.count_documents(doc! {}).await?;
    let total_chapters = chapters_collection.count_documents(doc! {}).await?;
    let avg_chapters_per_book = if total_books > 0 {
        total_chapters as f64 / total_books as f64
    } else {
        0.0
    };

    // AI Usage Metrics
    let total_ai_chapters = total_chapters as u64;
    let total_estimated_duration: u64 = chapters_collection
        .aggregate(vec![
            doc! { "$group": { "_id": null, "total_duration": { "$sum": "$estimated_duration" } } },
        ])
        .await?
        .next()
        .await
        .and_then(|doc| doc.ok()?.get_i64("total_duration").ok())
        .unwrap_or(0) as u64;

    let avg_gen_time = if total_ai_chapters > 0 {
        total_estimated_duration as f64 / total_ai_chapters as f64
    } else {
        0.0
    };

    let success_rate = 100.0;

    // Trending Topic
    let trending_topic = books_collection
        .aggregate(vec![
            doc! { "$group": { "_id": "$main_topic", "count": { "$sum": 1 } } },
            doc! { "$sort": { "count": -1 } },
            doc! { "$limit": 1 },
        ])
        .await?
        .next()
        .await
        .and_then(|doc| doc.ok()?.get_str("_id").ok().map(|s| s.to_string()))
        .unwrap_or_else(|| "Unknown".to_string());

    // Projected Growth
    let monthly_book_growth = books_collection
        .aggregate(vec![
            doc! { "$group": {
                "_id": { "month": { "$month": "$created_at" }, "year": { "$year": "$created_at" } },
                "count": { "$sum": 1 }
            }},
            doc! { "$sort": { "_id.year": 1, "_id.month": 1 } },
        ])
        .await?;

    let growth_rates: Vec<f64> = monthly_book_growth
        .collect::<Vec<_>>()
        .await
        .windows(2)
        .filter_map(|window| {
            if let (Ok(prev), Ok(curr)) = (window[0].as_ref(), window[1].as_ref()) {
                let prev_count = prev
                    .get_document("count")
                    .unwrap_or(&doc! {})
                    .get_i32("count")
                    .unwrap_or(1) as f64;
                let curr_count = curr
                    .get_document("count")
                    .unwrap_or(&doc! {})
                    .get_i32("count")
                    .unwrap_or(1) as f64;
                Some(((curr_count - prev_count) / prev_count) * 100.0)
            } else {
                None
            }
        })
        .collect();

    let projected_growth = growth_rates.last().cloned().unwrap_or(0.0);

    Ok(SuccessResponse {
        status: "success".into(),
        data: AnalyticsData {
            engagement: EngagementStats {
                total_books,
                total_chapters,
                avg_chapters_per_book,
            },
            ai_usage: AIUsageStats {
                total_ai_chapters,
                avg_gen_time,
                success_rate,
            },
            predictions: PredictiveStats {
                trending_genre: trending_topic,
                projected_growth,
            },
        },
    })
}
