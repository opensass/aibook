use crate::components::dashboard::books::list::CachedBooksData;
use crate::components::dashboard::books::list::CACHE_KEY;
use crate::components::dashboard::books::list::CACHE_TIMEOUT;
use crate::components::dashboard::books::read::CachedChaptersData;
use crate::components::dashboard::books::read::CHAPTERS_CACHE_KEY;
use crate::components::dashboard::books::read::CHAPTERS_CACHE_TIMEOUT;
use crate::server::book::controller::get_books_for_user;
use crate::server::book::controller::get_chapters_for_book;
use crate::server::book::model::Book;
use crate::server::book::model::Chapter;
use crate::server::book::request::GetBooksForUserRequest;
use crate::server::book::request::GetChaptersContentRequest;
use crate::server::conversation::controller::get_messages;
use crate::server::conversation::controller::save_message_to_db;
use crate::server::conversation::controller::send_query_to_gemini;
use crate::server::conversation::model::Message;
use crate::server::conversation::request::GetMessagesRequest;
use crate::server::conversation::request::SendQueryRequest;
use gloo_storage::Storage;

use bson::oid::ObjectId;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::LocalStorage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedMessagesData {
    pub conversation: String,
    pub messages: Vec<Message>,
    pub timestamp: i64,
}

pub const MESSAGES_CACHE_KEY: &str = "messages_cache";
pub const MESSAGES_CACHE_TIMEOUT: i64 = 2 * 60 * 60;

fn truncate(text: String, max_length: usize) -> String {
    if text.len() > max_length {
        format!("{}...", &text[0..max_length])
    } else {
        text.to_string()
    }
}

#[component]
pub fn ChatPanel(conversation_id: Signal<ObjectId>, user_token: Signal<String>) -> Element {
    let mut messages = use_signal(Vec::<Message>::new);
    let mut input_query = use_signal(|| "".to_string());
    let mut selected_book = use_signal(|| None::<Book>);
    let mut selected_chapter = use_signal(|| None::<Chapter>);
    let mut chapters = use_signal(Vec::<Chapter>::new);
    let mut books = use_signal(Vec::<Book>::new);
    let mut thinking = use_signal(|| false);
    let mut loading = use_signal(|| false);

    let _ = use_resource(move || async move {
        let now = Utc::now().timestamp();

        if let Ok(cached_data) = LocalStorage::get::<CachedBooksData>(CACHE_KEY) {
            if now - cached_data.timestamp < CACHE_TIMEOUT {
                // loading.set(false);
                books.set(cached_data.data.clone());
                if let Some(first_book) = cached_data.data.first() {
                    selected_book.set(Some(first_book.clone()));
                }
                return;
            }
        }

        match get_books_for_user(GetBooksForUserRequest {
            token: user_token(),
        })
        .await
        {
            Ok(response) => {
                let cached_data = CachedBooksData {
                    data: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CACHE_KEY, &cached_data);

                // loading.set(false);
                books.set(response.data.clone());
                if let Some(first_book) = response.data.first() {
                    selected_book.set(Some(first_book.clone()));
                }
            }
            Err(_) => {
                // loading.set(false);
            }
        }
    });

    use_effect(move || {
        let book_id = selected_book().unwrap_or_default().id.to_string();
        spawn(async move {
            let now = Utc::now().timestamp();

            if let Ok(cached_data) = LocalStorage::get::<CachedChaptersData>(CHAPTERS_CACHE_KEY) {
                if cached_data.book_id == book_id
                    && now - cached_data.timestamp < CHAPTERS_CACHE_TIMEOUT
                {
                    // loading.set(false);
                    chapters.set(cached_data.data.clone());
                    if let Some(first_chapter) = cached_data.data.first() {
                        selected_chapter.set(Some(first_chapter.clone()));
                    }
                    return;
                }
            }

            if let Ok(response) = get_chapters_for_book(GetChaptersContentRequest {
                book_id: book_id.clone(),
            })
            .await
            {
                // loading.set(false);
                chapters.set(response.data.clone());

                let cached_data = CachedChaptersData {
                    book_id: book_id.clone(),
                    data: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CHAPTERS_CACHE_KEY, &cached_data);

                if let Some(first_chapter) = response.data.first() {
                    selected_chapter.set(Some(first_chapter.clone()));
                }
            } else {
                // loading.set(true);
            }
        });
    });

    use_effect(move || {
        let conversation_id = conversation_id();
        spawn(async move {
            let now = Utc::now().timestamp();

            if let Ok(cached_data) = LocalStorage::get::<CachedMessagesData>(MESSAGES_CACHE_KEY) {
                if cached_data.conversation == conversation_id.to_string()
                    && now - cached_data.timestamp < MESSAGES_CACHE_TIMEOUT
                {
                    loading.set(false);
                    messages.set(cached_data.messages.clone());
                    return;
                }
            }

            if let Ok(response) = get_messages(GetMessagesRequest {
                token: user_token(),
                conversation_id: conversation_id,
            })
            .await
            {
                loading.set(false);
                messages.set(response.data.clone());

                let cached_data = CachedMessagesData {
                    conversation: conversation_id.to_string(),
                    messages: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(MESSAGES_CACHE_KEY, &cached_data);
            } else {
                loading.set(true);
            }
        });
    });

    let mut handle_send_query = {
        move || {
            if !input_query().is_empty()
                && selected_book().is_some()
                && selected_chapter().is_some()
            {
                thinking.set(true);
                let query_text = input_query();
                let book = selected_book().unwrap();
                let chapter = selected_chapter().unwrap();

                let user_message = Message {
                    id: ObjectId::new(),
                    conversation: conversation_id(),
                    sender: "user".to_string(),
                    content: query_text.clone(),
                    timestamp: Utc::now(),
                };

                let mut current_messages = messages();
                current_messages.push(user_message.clone());
                messages.set(current_messages);

                spawn(async move {
                    let response = send_query_to_gemini(SendQueryRequest {
                        query: query_text,
                        book: book.id.to_string(),
                        chapter: chapter.id.to_string(),
                        conversation_id: conversation_id(),
                        model: "gemini-2.0-flash".to_string(),
                        token: user_token(),
                    })
                    .await;

                    match response {
                        Ok(resp_message) => {
                            let mut current_messages = messages();
                            current_messages.push(resp_message.data);
                            thinking.set(false);
                            messages.set(current_messages);
                        }
                        Err(err) => {
                            dioxus_logger::tracing::error!("{:?}", err);
                            thinking.set(false);
                        }
                    }
                    save_message_to_db(user_message).await.unwrap();
                });

                input_query.set("".to_string());
            }
        }
    };
    let mut handle_book_change = move |book_id: String| {
        for book in books().into_iter() {
            if book.id.to_string() == book_id {
                selected_book.set(Some(book.clone()));

                spawn({
                    async move {
                        if let Ok(response) = get_chapters_for_book(GetChaptersContentRequest {
                            book_id: book.id.to_string(),
                        })
                        .await
                        {
                            chapters.set(response.data.clone());

                            if let Some(first_chapter) = response.data.first() {
                                selected_chapter.set(Some(first_chapter.clone()));
                            }
                        }
                    }
                });

                break;
            }
        }
    };

    rsx! {
        div {
            class: "flex flex-col h-full dark:bg-gray-900 dark:text-white bg-white text-gray-900",
            div {
                class: "flex flex-col md:flex-row md:space-x-4 p-4 border-b border-gray-300 dark:border-gray-700",

                select {
                    class: "p-2 rounded-lg mb-2 md:mb-0 flex-grow w-full md:w-auto truncate dark:bg-gray-700 dark:text-white bg-gray-100 text-black",
                    onchange: move |evt| handle_book_change(evt.value()),
                    option { value: "", "Select a book" },
                    for book in books().iter() {
                        option { value: "{book.id}", "{truncate(book.title.clone(), 20)}" }
                    }
                }

                select {
                    class: "p-2 rounded-lg flex-grow w-full md:w-auto truncate dark:bg-gray-700 dark:text-white bg-gray-100 text-black",
                    onchange: move |evt| selected_chapter.set(
                        chapters().iter().find(|chapter| chapter.id.to_string() == evt.value()).cloned()
                    ),
                    option { value: "", "Select a chapter" },
                    for chapter in chapters().iter() {
                        option { value: "{chapter.id}", "{truncate(chapter.title.clone(), 20)}" }
                    }
                }
            }

            div {
                class: "flex flex-col sm:flex-row items-center p-4 space-y-3 sm:space-y-0 sm:space-x-3 border-b border-gray-300 dark:border-gray-700",

                input {
                    class: "flex-1 p-2 rounded-lg border w-full dark:bg-gray-700 dark:text-white dark:border-gray-600 border-gray-300",
                    r#type: "text",
                    placeholder: "Type your query here...",
                    value: "{input_query}",
                    oninput: move |evt| input_query.set(evt.value()),
                    onkeypress: move |evt| {
                        if evt.key() == Key::Enter {
                            handle_send_query();
                        }
                    }
                }

                button {
                    class: "w-full sm:w-auto p-2 rounded-lg bg-blue-500 text-white hover:bg-blue-600",
                    onclick: move |_| handle_send_query(),
                    "Send"
                }
            }

            div {
                class: "flex-grow overflow-y-auto p-4 space-y-3",

                for message in messages().iter() {
                    div {
                        class: if message.sender == "user" { "text-right" } else { "text-left" },
                        div {
                            class: format!(
                                "inline-block px-4 py-2 rounded-lg {} max-w-full md:max-w-2/3",
                                if message.sender == "user" {
                                    "bg-blue-500 text-white"
                                } else {
                                    "bg-gray-300 dark:bg-gray-700 text-black dark:text-white"
                                }
                            ),
                            if message.sender == "user" {
                                div {
                                    "{message.content}",
                                }
                            }
                            else {
                                div {
                                    dangerous_inner_html: message.content.clone(),
                                }
                            }
                        }
                    }
                }
                if thinking() {
                    Thinking {}
                }
            }
        }
    }
}

#[component]
pub fn Thinking() -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-2 text-gray-600 dark:text-gray-400 font-medium",

            span { "🤔 Thinking" },

            svg {
                width: "60",
                height: "20",
                view_box: "0 0 60 20",
                xmlns: "http://www.w3.org/2000/svg",

                circle {
                    cx: "10",
                    cy: "10",
                    r: "2",
                    fill: "currentColor",
                    animate {
                        attribute_name: "opacity",
                        to: "1",
                        dur: "1s",
                        begin: "0s",
                        repeat_count: "indefinite",
                    }
                }
                circle {
                    cx: "30",
                    cy: "10",
                    r: "2",
                    fill: "currentColor",
                    animate {
                        attribute_name: "opacity",
                        to: "1",
                        dur: "1s",
                        begin: "0.3s",
                        repeat_count: "indefinite",
                    }
                }
                circle {
                    cx: "50",
                    cy: "10",
                    r: "2",
                    fill: "currentColor",
                    animate {
                        attribute_name: "opacity",
                        to: "1",
                        dur: "1s",
                        begin: "0.6s",
                        repeat_count: "indefinite",
                    }
                }
            }
        }
    }
}
