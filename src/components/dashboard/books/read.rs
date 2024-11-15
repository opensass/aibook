use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::server::book::controller::get_chapters_for_book;
use crate::server::book::model::Chapter;
use crate::server::book::request::GetChaptersContentRequest;
use crate::theme::Theme;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedChaptersData {
    pub book_id: String,
    pub data: Vec<Chapter>,
    pub timestamp: i64,
}

pub const CHAPTERS_CACHE_KEY: &str = "chapters_cache";
pub const CHAPTERS_CACHE_TIMEOUT: i64 = 2 * 60 * 60;

#[component]
pub fn ReadBookPanel(book_id: String) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    let mut selected_chapter = use_signal(|| None::<Chapter>);
    let mut chapters = use_signal(Vec::<Chapter>::new);
    let mut loading = use_signal(|| true);

    use_effect(move || {
        let book_id_cloned = book_id.clone();
        spawn(async move {
            let now = Utc::now().timestamp();

            if let Ok(cached_data) = LocalStorage::get::<CachedChaptersData>(CHAPTERS_CACHE_KEY) {
                if cached_data.book_id == book_id_cloned
                    && now - cached_data.timestamp < CHAPTERS_CACHE_TIMEOUT
                {
                    loading.set(false);
                    chapters.set(cached_data.data.clone());
                    if let Some(first_chapter) = cached_data.data.first() {
                        selected_chapter.set(Some(first_chapter.clone()));
                    }
                    return;
                }
            }

            if let Ok(response) = get_chapters_for_book(GetChaptersContentRequest {
                book_id: book_id_cloned.clone(),
            })
            .await
            {
                loading.set(false);
                chapters.set(response.data.clone());

                let cached_data = CachedChaptersData {
                    book_id: book_id_cloned.clone(),
                    data: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CHAPTERS_CACHE_KEY, &cached_data);

                if let Some(first_chapter) = response.data.first() {
                    selected_chapter.set(Some(first_chapter.clone()));
                }
            } else {
                loading.set(true);
            }
        });
    });

    let mut handle_chapter_click = {
        let mut selected_chapter = selected_chapter.clone();
        move |chapter: Chapter| {
            selected_chapter.set(Some(chapter));
        }
    };

    rsx! {
        div {
            class: format!("flex h-full {}", if dark_mode { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),

            div {
                class: "md:w-1/3 lg:w-1/4 sm:w-1/6 p-4 border-r border-blue-300",
                ul {
                    class: "space-y-4",
                    for (index, chapter) in chapters().into_iter().enumerate() {
                        li {
                            class: format!("flex items-center p-3 rounded-lg cursor-pointer {}",
                                if chapter.id == selected_chapter().unwrap().id {
                                    "bg-gray-500 text-white font-semibold"
                                } else {
                                    "hover:bg-gray-200 dark:hover:bg-dark-800"
                                }),
                            onclick: move |_| handle_chapter_click(chapter.clone()),
                            div {
                                class: "w-8 h-8 flex items-center justify-center rounded-full border-2 border-blue-500 mr-4",
                                "{index + 1}"
                            },

                            div {
                                class: "flex-1 hidden sm:block",
                                h4 { class: "text-lg", "{chapter.title}" }
                                p { class: "text-sm text-blue-500", "{chapter.estimated_duration} minutes" }
                            }
                        }
                    }
                }
            }

            div {
                class: "flex-1 p-6 overflow-y-auto",
                if let Some(chapter) = selected_chapter() {
                    h2 { class: "text-2xl font-bold mb-4", "{chapter.title}" }
                    p { class: "text-sm text-blue-500 mb-6", "{chapter.estimated_duration} minutes" }
                    div {
                        class: "prose dark:prose-invert",
                        dangerous_inner_html: if chapter.html.is_empty() {chapter.markdown} else {chapter.html},
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
                            span { "Loading book's chapters..." }
                        } else {
                            Spinner {
                                aria_label: "Loading spinner".to_string(),
                                size: SpinnerSize::Md,
                                dark_mode: true,
                            }
                            span { "No chapters found! Generating..." }
                        }
                    }
                }
            }
        }
    }
}
