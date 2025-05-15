use crate::components::dashboard::books::list::CachedBooksData;
use crate::components::dashboard::books::list::CACHE_KEY;
use crate::components::dashboard::fields::select::SelectField;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::server::book::controller::generate_book_outline;
use crate::server::book::controller::generate_chapter_content;
use crate::server::book::request::GenerateBookRequest;
use crate::server::book::request::GenerateChapterContentRequest;
use chrono::Duration;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use input_rs::dioxus::Input;

pub fn validate_input(field: String) -> bool {
    !&field.is_empty()
}

#[component]
pub fn CreateBookPanel(user_token: Signal<String>) -> Element {
    let title = use_signal(|| "".to_string());
    let subtitle = use_signal(|| "".to_string());
    let model = use_signal(|| "gemini-1.5-flash".to_string());
    let subtopics = use_signal(|| "3".to_string());
    let chapters = use_signal(|| "5".to_string());
    let language = use_signal(|| "English".to_string());
    let max_length = use_signal(|| "1000".to_string());

    let title_valid = use_signal(|| true);
    let subtitle_valid = use_signal(|| true);
    let subtopics_valid = use_signal(|| true);
    let language_valid = use_signal(|| true);
    let maxlen_valid = use_signal(|| true);
    let chapters_valid = use_signal(|| true);
    let mut loading = use_signal(|| false);
    let _form_error = use_signal(|| None::<String>);

    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let handle_submit = move |e: Event<FormData>| {
        e.stop_propagation();
        let title_value = title().clone();
        let subtitle_value = subtitle().clone();
        loading.set(true);

        if !validate_input(title_value) || !validate_input(subtitle_value) {
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
        div { class: "p-4 dark:bg-gray-800 dark:text-white bg-white text-gray-900",
            h2 { class: "text-xl font-semibold mb-4", "Generate" }
            form { class: "space-y-4",
                onsubmit: handle_submit,
                Input {
                    r#type: "text",
                    label: "Title",
                    handle: title,
                    placeholder: "Title",
                    error_message: "Title can't be blank!",
                    required: true,
                    valid_handle: title_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                    input_class: {if title_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    }},
                    error_class: "text-red-500 text-sm mt-1",
                }
                Input {
                    r#type: "text",
                    label: "Subtitle",
                    handle: subtitle,
                    placeholder: "Subtitle",
                    error_message: "Subtitle can't be blank!",
                    required: true,
                    valid_handle: subtitle_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                    input_class: {if subtitle_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    }},
                    error_class: "text-red-500 text-sm mt-1",
                }
                SelectField { label: "Model", options: vec!["gemini-pro", "gemini-1.0-pro", "gemini-1.5-pro", "gemini-1.5-flash"], selected: model }
                Input {
                    r#type: "number",
                    label: "Subtopics per Chapter",
                    handle: subtopics,
                    placeholder: "Subtopics",
                    error_message: "Subtopics can't be blank!",
                    required: true,
                    valid_handle: subtopics_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                    input_class: {if subtopics_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    }},
                    error_class: "text-red-500 text-sm mt-1",
                }
                Input {
                    r#type: "number",
                    label: "Chapters",
                    handle: chapters,
                    placeholder: "Chapters",
                    error_message: "Chapters can't be blank!",
                    required: true,
                    valid_handle: chapters_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                    input_class: {if chapters_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    }},
                    error_class: "text-red-500 text-sm mt-1",
                }
                Input {
                    r#type: "text",
                    label: "Language",
                    handle: language,
                    placeholder: "Language",
                    error_message: "Language can't be blank!",
                    required: true,
                    valid_handle: language_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                    input_class: {if language_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    }},
                    error_class: "text-red-500 text-sm mt-1",
                }
                Input {
                    r#type: "number",
                    label: "Max Length",
                    handle: max_length,
                    placeholder: "Max Length",
                    error_message: "Language can't be blank!",
                    required: true,
                    valid_handle: maxlen_valid,
                    validate_function: validate_input,
                    class: "field mb-6",
                    field_class: "validate-input mb-6",
                    label_class: "block dark:text-gray-300 text-sm font-medium text-gray-700",
                    input_class: {if maxlen_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                    }},
                    error_class: "text-red-500 text-sm mt-1",
                }
                // if let Some(error) = &form_error() {
                //     p { class: "text-red-600", "{error}" }
                // }
                button {
                    class: "flex items-center space-x-2 bg-blue-500 text-white px-4 py-2 rounded dark:bg-blue-600",
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
