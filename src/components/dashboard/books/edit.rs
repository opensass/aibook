use crate::server::auth::controller::about_me;
use crate::server::book::controller::{
    extend_text, regenerate_text, summarize_text, update_book_content,
};
use crate::server::book::model::Book;
use crate::server::book::request::{AIRequest, UpdateBookContentRequest};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;

use web_sys::window;

#[component]
pub fn EditBookContentPanel(book_id: String) -> Element {
    let navigator = use_navigator();

    let book = use_signal(|| Option::<Book>::None);
    let book_id = use_signal(|| book_id);
    let mut user_token = use_signal(|| "".to_string());
    let content = use_signal(|| "".to_string());
    let mut error_message = use_signal(|| None::<String>);
    let mut show_ai_modal = use_signal(|| false);
    let mut selected_text = use_signal(|| "".to_string());

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(_) => {
                        user_token.set(token);
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });

    // let _ = use_resource(move || {
    //     let value = book_id();
    //     async move {
    //         match get_book_for_user(GetBookForUserRequest {
    //             token: user_token(),
    //             book_id: value,
    //         })
    //         .await
    //         {
    //             Ok(response) => {
    //                 book.set(Some(response.data.clone()));
    //                 content.set(response.data.content);
    //                 error_message.set(None);
    //             },
    //             Err(err) => error_message.set(Some(format!("Error fetching book: {:?}", err))),
    //         }
    //     }
    // });

    let handle_save_content = move |_| {
        let content_value = content().clone();
        spawn({
            let user_token = user_token();
            async move {
                match update_book_content(UpdateBookContentRequest {
                    token: user_token,
                    book_id: book_id(),
                    new_content: content_value,
                })
                .await
                {
                    Ok(_) => error_message.set(Some("Content saved successfully".to_string())),
                    Err(err) => error_message.set(Some(format!("Error saving content: {:?}", err))),
                }
            }
        });
    };

    let handle_text_selection = move |_e: Event<MouseData>| {
        let selected = window()
            .expect("Window must exist")
            .get_selection()
            .expect("Failed to get selection");
        let select_text: String = selected
            .expect("selected text must exist")
            .to_string()
            .into();
        if !select_text.is_empty() {
            selected_text.set(select_text.clone());
            show_ai_modal.set(true);
        }
    };

    rsx! {
        div {
            class: "p-4 dark:bg-gray-800 dark:text-white bg-white text-gray-900",
            h2 { class: "text-xl font-semibold mb-4", "Edit Book Content" }
            if let Some(error) = error_message() {
                p { class: "text-red-600", "{error}" }
            }

            if let Some(book) = book() {
                div {
                    class: "mb-4",
                    "Editing: {book.main_topic.clone().unwrap_or(\"Untitled\".to_string())}"
                }
                button {
                    class: "mt-4 bg-blue-500 text-white px-4 py-2 rounded dark:bg-blue-600",
                    onclick: handle_save_content,
                    "Save Changes"
                }

                if show_ai_modal() {
                    div {
                        class: "modal-overlay",
                        onclick: move |_| show_ai_modal.set(false),
                        div {
                            class: "modal-content",
                            h3 { class: "text-lg font-semibold", "AI Options" }
                            // p { "Selected text: {selected_text()}" }
                            button {
                                class: "bg-green-500 text-white px-4 py-2 rounded",
                                onclick: move |_| apply_ai_action(
                                    "summarize",
                                    user_token(),
                                    selected_text(),
                                    content.clone(),
                                    error_message.clone(),
                                    show_ai_modal.clone()
                                ),
                                "Summarize"
                            }
                            button {
                                class: "bg-yellow-500 text-white px-4 py-2 rounded ml-2",
                                onclick: move |_| apply_ai_action(
                                    "regenerate",
                                    user_token(),
                                    selected_text(),
                                    content.clone(),
                                    error_message.clone(),
                                    show_ai_modal.clone()
                                ),
                                "Regenerate"
                            }
                            button {
                                class: "bg-blue-500 text-white px-4 py-2 rounded ml-2",
                                onclick: move |_| apply_ai_action(
                                    "extend",
                                    user_token(),
                                    selected_text(),
                                    content.clone(),
                                    error_message.clone(),
                                    show_ai_modal.clone()
                                ),
                                "Extend"
                            }
                        }
                    }
                }
                div {
                    class: "editable-content relative mt-2",
                    onmousemove: handle_text_selection,
                    contenteditable: "true",
                    {content()}
                }

            } else {
                p { "Loading book content..." }
            }
        }
    }
}

fn apply_ai_action(
    action: &'static str,
    user_token: String,
    selected_text_value: String,
    mut content: Signal<String>,
    mut error_message: Signal<Option<String>>,
    mut show_ai_modal: Signal<bool>,
) {
    spawn({
        async move {
            let ai_request = AIRequest {
                token: user_token.clone(),
                text: selected_text_value.clone(),
            };
            let response = match action {
                "summarize" => summarize_text(ai_request).await,
                "regenerate" => regenerate_text(ai_request).await,
                "extend" => extend_text(ai_request).await,
                _ => Err(ServerFnError::new("Invalid action")),
            };

            match response {
                Ok(new_text) => {
                    let updated_content = content().replace(&selected_text_value, &new_text.data);
                    tracing::error!("{}", updated_content);
                    content.set("".to_string());
                }
                Err(err) => {
                    error_message.set(Some(format!("AI action failed: {:?}", err)));
                }
            }

            show_ai_modal.set(false);
        }
    });
}
