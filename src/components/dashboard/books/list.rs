use crate::components::dashboard::analytics::AnalyticsPage;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::router::Route;
use crate::server::book::controller::get_books_for_user;
use crate::server::book::model::Book;
use crate::server::book::request::GetBooksForUserRequest;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedBooksData {
    pub data: Vec<Book>,
    pub timestamp: i64,
}

pub const CACHE_KEY: &str = "books_cache";
pub const CACHE_TIMEOUT: i64 = 2 * 60 * 60;

#[component]
pub fn BooksPanel(user_token: Signal<String>) -> Element {
    let mut books = use_signal(Vec::new);
    let mut displayed_books = use_signal(Vec::new);
    let mut loading = use_signal(|| true);
    let mut search_query = use_signal(String::new);

    let _ = use_resource(move || async move {
        let now = Utc::now().timestamp();

        if let Ok(cached_data) = LocalStorage::get::<CachedBooksData>(CACHE_KEY) {
            if now - cached_data.timestamp < CACHE_TIMEOUT {
                loading.set(false);
                books.set(cached_data.data.clone());
                displayed_books.set(cached_data.data);
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

                loading.set(false);
                books.set(response.data.clone());
                displayed_books.set(response.data);
            }
            Err(_) => {
                loading.set(false);
            }
        }
    });

    let mut filter_books = move || {
        let query = search_query().to_lowercase();

        let filtered_books = books()
            .iter()
            .filter(|book| {
                let matches_query = if query.is_empty() {
                    true
                } else {
                    let title_matches = book.title.to_lowercase().contains(&query);
                    let subtitle_matches = book
                        .subtitle
                        .as_deref()
                        .map(|s| s.to_lowercase().contains(&query))
                        .unwrap_or(false);
                    title_matches || subtitle_matches
                };

                matches_query
            })
            .cloned()
            .collect::<Vec<_>>();

        displayed_books.set(filtered_books);
    };

    rsx! {
        div {
            AnalyticsPage {}
            div {
                div {
                    class: "w-full md:w-1/3 pb-4 mb-4 md:mb-0 flex flex-col gap-8",

                    div {
                        h3 { class: "text-2xl font-bold mb-4", "Search" }
                        input {
                            class: "mt-1 block w-full p-2 border rounded-md shadow-sm dark:bg-gray-900",
                            placeholder: "Search by title...",
                            value: "{search_query()}",
                            oninput: move |e| {
                                search_query.set(e.value());
                                filter_books();
                            },
                        }
                    }
                }
                h2 { class: "text-xl font-semibold mb-4", "All Books" }
                if displayed_books.len() > 0 {
                    div {
                        class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                        for book in displayed_books() {
                            Link {
                                to: Route::ReadBook { id: book.id.to_string() },
                                class: "p-4 shadow rounded-lg dark:bg-gray-700 bg-gray-100",
                                img {
                                    src: book.cover.as_deref().unwrap_or("/path/to/default-cover.jpg"),
                                    alt: "Book cover",
                                    class: "w-full h-48 object-cover rounded-md mb-4"
                                }
                                h3 {
                                    class: "text-lg font-bold mb-2",
                                    "{book.main_topic.clone().unwrap_or(\"Untitled\".to_string())}"
                                }
                                p {
                                    class: "text-sm text-gray-500 mb-2",
                                    "{book.created_at.format(\"%B %d, %Y\")} · {book.title.len() / 7000} min read"
                                }
                                p {
                                    class: format!(
                                        "text-sm {}",
                                        if book.completed { "text-green-600" } else { "text-red-600" }
                                    ),
                                    if book.completed { "Completed" } else { "In Progress" }
                                }
                                p {
                                    class: "mt-2 text-sm text-gray-700",
                                    "{book.title.chars().take(30).collect::<String>()}..."
                                }
                            }
                        }
                    }
                } else {
                    p {
                        class: "flex items-center space-x-2 px-4 py-2 rounded",
                        if loading() {
                            Spinner {
                                aria_label: "Loading spinner".to_string(),
                                size: SpinnerSize::Md,
                                dark_mode: true,
                            }
                            span { "Loading books..." }
                        } else {
                            span { "No books match your search filter." }
                        }
                    }
                }
            }
        }
    }
}
