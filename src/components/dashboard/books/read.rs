use crate::router::Route;
use crate::server::book::controller::get_chapters_for_book;
use crate::server::book::model::{Book, Chapter};
use crate::server::book::request::GetChaptersContentRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;
use gloo_storage::SessionStorage;

#[component]
pub fn ReadBookPanel(book_id: String) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut selected_chapter = use_signal(|| None::<Chapter>);
    let mut chapters = use_signal(Vec::<Chapter>::new);

    use_effect(move || {
        let value = book_id.clone();
        spawn(async move {
            if let Ok(response) = get_chapters_for_book(GetChaptersContentRequest {
                book_id: value.clone(),
            })
            .await
            {
                chapters.set(response.data.clone());
                if let Some(first_chapter) = response.data.first() {
                    selected_chapter.set(Some(first_chapter.clone()));
                }
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
                    p { "Loading chapter content..." }
                }
            }
        }
    }
}