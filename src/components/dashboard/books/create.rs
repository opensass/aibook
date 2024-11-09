use crate::components::dashboard::fields::input::InputField;
use crate::components::dashboard::fields::number::NumberField;
use crate::components::dashboard::fields::select::SelectField;
use crate::server::book::controller::generate_book_outline;
use crate::server::book::controller::generate_chapter_content;
use crate::server::book::controller::store_book;
use crate::server::book::request::GenerateBookRequest;
use crate::server::book::request::GenerateChapterContentRequest;
use crate::server::book::request::StoreBookRequest;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

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
    let mut form_error = use_signal(|| None::<String>);

    let validate_title = |title: &str| !title.is_empty();
    let validate_subtitle = |subtitle: &str| !subtitle.is_empty();
    let validate_language = |language: &str| !language.is_empty();

    let handle_submit = move |e: Event<FormData>| {
        e.stop_propagation();
        let title_value = title().clone();
        let subtitle_value = subtitle().clone();

        if !validate_title(&title_value) || !validate_subtitle(&subtitle_value) {
            form_error.set(Some("Title and subtitle are required.".to_string()));
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
                            for chapter in response.data {
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
                                    Ok(_) => println!("Book created successfully!"),
                                    Err(e) => {
                                        form_error.set(Some(format!("Failed to store book: {}", e)))
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            form_error.set(Some(format!("Failed to generate content: {}", e)))
                        }
                    }
                }
            }
        });
    };

    rsx! {
        div { class: format!("p-4 {}", if dark_mode { "bg-gray-800 text-white" } else { "bg-white text-gray-900" }),
            h2 { class: "text-xl font-semibold mb-4", "Create Book" }
            form { class: "space-y-4",
                onsubmit: handle_submit,
                InputField { label: "Title", value: title, is_valid: title_valid, validate: validate_title }
                InputField { label: "Subtitle", value: subtitle, is_valid: subtitle_valid, validate: validate_subtitle }
                SelectField { label: "Model", options: vec!["gemini-pro", "gemini-1.0-pro", "gemini-1.5-pro", "gemini-1.5-flash"], selected: model }
                NumberField { label: "Subtopics per Chapter", value: subtopics }
                NumberField { label: "Chapters", value: chapters }
                InputField { label: "Language", value: language, is_valid: language_valid, validate: validate_language }
                NumberField { label: "Max Length", value: max_length }
                if let Some(error) = &form_error() {
                    p { class: "text-red-600", "{error}" }
                }
                button {
                    class: format!("bg-blue-500 text-white px-4 py-2 rounded {}", if dark_mode { "dark:bg-blue-600" } else { "" }),
                    r#type: "submit",
                    "Create Book"
                }
            }
        }
    }
}
