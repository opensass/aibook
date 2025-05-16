pub(crate) mod author;
pub(crate) mod card;
pub(crate) mod rating;

use crate::components::testimonial::author::AuthorInfo;
use crate::components::testimonial::rating::StarRating;
use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[derive(Props, Clone, PartialEq)]
pub struct TestimonialData {
    quote: String,
    author_name: String,
    author_title: String,
    author_image: Asset,
    company_logo: Asset,
    star_images: Vec<&'static str>,
}

#[component]
pub fn Testimonial() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    let testimonials = vec![
        TestimonialData {
            quote: i18n().t("testimonial.0.quote"),
            author_name: i18n().t("testimonial.0.author_name"),
            author_title: i18n().t("testimonial.0.author_title"),
            author_image: asset!("/assets/shakespeare.webp"),
            company_logo: asset!("/assets/shakespeare_logo.webp"),
            star_images: vec!["fas fa-star"; 5],
        },
        TestimonialData {
            quote: i18n().t("testimonial.1.quote"),
            author_name: i18n().t("testimonial.1.author_name"),
            author_title: i18n().t("testimonial.1.author_title"),
            author_image: asset!("/assets/neo.webp"),
            company_logo: asset!("/assets/matrix_logo.webp"),
            star_images: vec!["fas fa-star"; 5],
        },
        TestimonialData {
            quote: i18n().t("testimonial.2.quote"),
            author_name: i18n().t("testimonial.2.author_name"),
            author_title: i18n().t("testimonial.2.author_title"),
            author_image: asset!("/assets/darth_vader.webp"),
            company_logo: asset!("/assets/empire_logo.webp"),
            star_images: vec!["fas fa-star"; 5],
        },
    ];

    let mut current_index = use_signal(|| 0);

    client! {
        let vec_len = testimonials.len();
        let mut eval = document::eval(
            r#"
            setInterval(() => {
                dioxus.send("");
            }, 5000)
            "#,
        );

        use_hook(|| {
            spawn(async move {
                loop {
                    let _ = eval.recv::<String>().await;
                    current_index.set((current_index() + 1) % vec_len);
                }
            })
        });
    }

    rsx! {
        section {
            id: "testimonial",
            class: "flex flex-col items-center justify-center min-h-screen p-8 dark:bg-gray-900 dark:text-white bg-white text-black",

            div { class: "flex flex-col items-center mb-8",
                h2 { class: "text-4xl font-bold text-center",
                    "{i18n().t(\"testimonial.title\")} ",
                    span { class: "bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent animate-pulse", "{i18n().t(\"testimonial.highlighted_word\")}" }
                }

                p { class: "mt-2 text-lg dark:text-gray-300 text-gray-700", "{i18n().t(\"testimonial.subtitle\")}" }
            }

            div { class: "flex items-center overflow-x-auto space-x-8 p-4",
                for (i, testimonial) in testimonials.iter().enumerate() {
                    div { class: format!("transition-transform duration-500 transform {}", if current_index() == i { "opacity-100 scale-100" } else { "opacity-50 scale-75 blur-sm" }),
                        div { class: "p-8 rounded-lg shadow-lg text-center max-w-sm border dark:border-gray-700 dark:bg-gray-800 bg-white border-gray-300",
                            StarRating { star_images: testimonial.star_images.clone() }
                            blockquote { class: "text-lg font-semibold", "{testimonial.quote}" }
                            AuthorInfo {
                                author_image: testimonial.author_image,
                                author_name: testimonial.author_name.clone(),
                                author_title: testimonial.author_title.clone(),
                                company_logo: testimonial.company_logo,
                            }
                        }
                    }
                }
            }

            div { class: "flex justify-center mt-4 space-x-2",
                for (i, _) in testimonials.iter().enumerate() {
                    div { class: format!("w-3 h-3 rounded-full {} transition-all duration-300", if current_index() == i { "bg-blue-500" } else { "bg-gray-400" }) }
                }
            }
        }
    }
}
