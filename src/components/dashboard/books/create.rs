use crate::components::dashboard::books::list::CachedBooksData;
use crate::components::dashboard::books::list::CACHE_KEY;
use crate::components::dashboard::fields::input::InputField;
use crate::components::dashboard::fields::number::NumberField;
use crate::components::dashboard::fields::select::SelectField;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::toast::manager::Toast;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::server::book::controller::generate_book_outline;
use crate::server::book::controller::generate_chapter_content;
use crate::server::book::controller::store_book;
use crate::server::book::request::GenerateBookRequest;
use crate::server::book::request::GenerateChapterContentRequest;
use crate::server::book::request::StoreBookRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use chrono::Duration;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};

#[component]
pub fn CreateBookPanel(user_token: Signal<String>) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let title = use_signal(|| "".to_string());
    let subtitle = use_signal(|| "".to_string());
    let model = use_signal(|| "gemini-1.5-flash".to_string());
    let subtopics = use_signal(|| 3);
    let chapters = use_signal(|| 5);
    let language = use_signal(|| "English".to_string());
    let max_length = use_signal(|| 1000);

    let title_valid = use_signal(|| true);
    let subtitle_valid = use_signal(|| true);
    let language_valid = use_signal(|| true);
    let mut loading = use_signal(|| false);
    let mut form_error = use_signal(|| None::<String>);

    let validate_title = |title: &str| !title.is_empty();
    let validate_subtitle = |subtitle: &str| !subtitle.is_empty();
    let validate_language = |language: &str| !language.is_empty();

    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let handle_submit = move |e: Event<FormData>| {
        e.stop_propagation();
        let title_value = title().clone();
        let subtitle_value = subtitle().clone();
        loading.set(true);

        if !validate_title(&title_value) || !validate_subtitle(&subtitle_value) {
            // form_error.set(Some("Title and subtitle are required.".to_string()));
            toasts_manager.set(
                toasts_manager()
                    .add_toast(
                        "Error".into(),
                        "Title and subtitle are required!".into(),
                        ToastType::Error,
                        Some(Duration::seconds(5)),
                    )
                    .clone(),
            );
            return;
        }

        spawn({
            async move {
                if !user_token().is_empty() {
                    match generate_book_outline(GenerateBookRequest {
                        title: title(),
                        token: user_token(),
                        subtitle: subtitle(),
                        model: model(),
                        subtopics: subtopics(),
                        chapters: chapters(),
                        language: language(),
                        max_length: max_length(),
                    })
                    .await
                    {
                        Ok(response) => {
                            let mut cached_data = LocalStorage::get::<CachedBooksData>(CACHE_KEY)
                                .unwrap_or(CachedBooksData {
                                    data: Vec::new(),
                                    timestamp: Utc::now().timestamp(),
                                });

                            cached_data.data.push(response.data.book);

                            let _ = LocalStorage::set(CACHE_KEY, &cached_data);
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Info".into(),
                                        "Book outline generated successfully!".into(),
                                        ToastType::Info,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Info".into(),
                                        "Generating chapters content...".into(),
                                        ToastType::Info,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                            for chapter in response.data.chapters {
                                match generate_chapter_content(GenerateChapterContentRequest {
                                    chapter_title: chapter.title,
                                    book_title: title(),
                                    main_topic: chapter.html,
                                    language: language(),
                                    model: model(),
                                    chapter_id: chapter.id,
                                })
                                .await
                                {
                                    Ok(_) => {
                                        toasts_manager.set(
                                            toasts_manager()
                                                .add_toast(
                                                    "Info".into(),
                                                    "Book generated successfully!".into(),
                                                    ToastType::Success,
                                                    Some(Duration::seconds(5)),
                                                )
                                                .clone(),
                                        );
                                        loading.set(false);
                                    }
                                    Err(e) => {
                                        // form_error.set(Some(format!("Failed to store book: {}", e)));
                                        let msg = e.to_string();
                                        let error_message = msg
                                            .splitn(2, "error running server function:")
                                            .nth(1)
                                            .unwrap_or("")
                                            .trim();
                                        toasts_manager.set(
                                            toasts_manager()
                                                .add_toast(
                                                    "Error".into(),
                                                    error_message.into(),
                                                    ToastType::Error,
                                                    Some(Duration::seconds(5)),
                                                )
                                                .clone(),
                                        );
                                        loading.set(false);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            // form_error.set(Some(format!("Failed to generate content: {}", e)));
                            let msg = e.to_string();
                            let error_message = msg
                                .splitn(2, "error running server function:")
                                .nth(1)
                                .unwrap_or("")
                                .trim();
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Error".into(),
                                        error_message.into(),
                                        ToastType::Error,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                            loading.set(false);
                        }
                    }
                }
            }
        });
    };

    rsx! {
        div { class: format!("p-4 {}", if dark_mode { "bg-gray-800 text-white" } else { "bg-white text-gray-900" }),
            h2 { class: "text-xl font-semibold mb-4", "Generate" }
            form { class: "space-y-4",
                onsubmit: handle_submit,
                InputField { label: "Title", value: title, is_valid: title_valid, validate: validate_title, required: true }
                InputField { label: "Subtitle", value: subtitle, is_valid: subtitle_valid, validate: validate_subtitle, required: true }
                SelectField { label: "Model", options: vec!["gemini-pro", "gemini-1.0-pro", "gemini-1.5-pro", "gemini-1.5-flash"], selected: model }
                NumberField { label: "Subtopics per Chapter", value: subtopics, required: true }
                NumberField { label: "Chapters", value: chapters, required: true }
                InputField { label: "Language", value: language, is_valid: language_valid, validate: validate_language, required: true }
                NumberField { label: "Max Length", value: max_length, required: true }
                // if let Some(error) = &form_error() {
                //     p { class: "text-red-600", "{error}" }
                // }
                button {
                    class: format!("flex items-center space-x-2 bg-blue-500 text-white px-4 py-2 rounded {}", if dark_mode { "bg-blue-600" } else { "" }),
                    r#type: "submit",
                    disabled: loading(),
                    if loading() {
                        Spinner {
                            aria_label: "Loading spinner".to_string(),
                            size: SpinnerSize::Md,
                            dark_mode: true,
                        }
                        span { "Generating..." }
                    } else {
                        span { "Generate" }
                    }
                }
            }
        }
    }
}
