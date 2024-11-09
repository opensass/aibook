use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::router::Route;
use crate::server::book::controller::get_books_for_user;
use crate::server::book::request::GetBooksForUserRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use chrono::Duration;
use dioxus::prelude::*;

#[component]
pub fn BooksPanel(user_token: Signal<String>) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let mut books = use_signal(Vec::new);
    let mut loading = use_signal(|| true);

    let _ = use_resource(move || async move {
        match get_books_for_user(GetBooksForUserRequest {
            token: user_token(),
        })
        .await
        {
            Ok(response) => {
                loading.set(false);
                books.set(response.data);
            }
            Err(err) => {
                loading.set(false);
            }
        }
    });

    rsx! {
        div {
            class: format!(
                "p-4 {}",
                if dark_mode { "bg-gray-800 text-white" } else { "bg-white text-gray-900" }
            ),
            h2 { class: "text-xl font-semibold mb-4", "Books" }

            if books.len() > 0 {
                div {
                    class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6",
                    for book in books() {
                        Link {
                            to: Route::ReadBook { id: book.id.to_string() },
                            class: format!(
                                "p-4 shadow rounded-lg {}",
                                if dark_mode { "bg-gray-700" } else { "bg-gray-100" }
                            ),
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
                    }
                }
            }
        }
    }
}
