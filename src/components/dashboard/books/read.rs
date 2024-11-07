use crate::server::auth::controller::about_me;
use crate::server::book::controller::get_book_for_user;
use crate::server::book::model::Book;
use crate::server::book::request::GetBookForUserRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;

#[component]
pub fn ReadBookPanel(book_id: String) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let navigator = use_navigator();

    let mut book = use_signal(|| Option::<Book>::None);
    let mut user_token = use_signal(|| "".to_string());
    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
                        user_token.set(token.clone());
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });

    let _ = use_resource(move || {
        let value = book_id.clone();
        async move {
            match get_book_for_user(GetBookForUserRequest {
                token: user_token(),
                book_id: value.clone(),
            })
            .await
            {
                Ok(response) => book.set(Some(response.data)),
                Err(err) => eprintln!("Error fetching book: {:?}", err),
            }
        }
    });

    rsx! {
        div {
            class: format!("p-4 {}", if dark_mode { "bg-gray-800 text-white" } else { "bg-white text-gray-900" }),
            h2 { class: "text-xl font-semibold mb-4", "Read Book" }

            if let Some(book) = book() {
                div {
                    class: "mt-4",
                    "Title: {book.main_topic.clone().unwrap_or(\"Untitled\".to_string())}"
                }
                div {
                    class: "mt-2",
                    h3 { class: "text-lg font-semibold", "Content:" }
                    div {
                        class: "book-content mt-2",
                        dangerous_inner_html: "{book.content}",
                    }
                }
            } else {
                p { "Loading book content..." }
            }
        }
    }
}
