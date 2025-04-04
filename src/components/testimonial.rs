pub(crate) mod author;
pub(crate) mod card;
pub(crate) mod rating;

use crate::components::testimonial::author::AuthorInfo;
use crate::components::testimonial::rating::StarRating;
use crate::theme::Theme;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TestimonialData {
    quote: &'static str,
    author_name: &'static str,
    author_title: &'static str,
    author_image: Asset,
    company_logo: Asset,
    star_images: Vec<&'static str>,
}

#[allow(unused_mut)]
#[component]
pub fn Testimonial() -> Element {
    let testimonials = vec![
        TestimonialData {
            quote: "AIbook writes poetry that would make any bard jealous. And it doesn’t even charge a royal fee!",
            author_name: "William Shakespeare",
            author_title: "Playwright",
            author_image: asset!("/assets/shakespeare.webp"),
            company_logo: asset!("/assets/shakespeare_logo.webp"),
            star_images: vec!["fas fa-star"; 5],
        },
        TestimonialData {
            quote: "I asked AIbook to write a novel. It wrote a sci-fi epic that somehow included me as the protagonist. I might be living in a simulation!",
            author_name: "Neo",
            author_title: "The One",
            author_image: asset!("/assets/neo.webp"),
            company_logo: asset!("/assets/matrix_logo.webp"),
            star_images: vec!["fas fa-star"; 5],
        },
        TestimonialData {
            quote: "AIbook practically writes my memoirs for me! Now I can focus on other pressing matters, like conquering the galaxy.",
            author_name: "Darth Vader",
            author_title: "Dark Lord of the Sith",
            author_image: asset!("/assets/darth_vader.webp"),
            company_logo: asset!("/assets/empire_logo.webp"),
            star_images: vec!["fas fa-star"; 5],
        },
    ];

    let dark_mode = use_context::<Signal<Theme>>();
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
            class: format!("flex flex-col items-center justify-center min-h-screen p-8 {}",
            if dark_mode() == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-black" }),

            div { class: "flex flex-col items-center mb-8",
                h2 { class: "text-4xl font-bold text-center",
                    "Trusted by ",
                    span { class: "bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent animate-pulse", "Millions" }
                }

                p { class: format!("mt-2 text-lg {}", if dark_mode() == Theme::Dark { "text-gray-300" } else { "text-gray-700" }), "Real Reviews from Real Users" }
            }

            div { class: "flex items-center overflow-x-auto space-x-8 p-4",
                for (i, testimonial) in testimonials.iter().enumerate() {
                    div { class: format!("transition-transform duration-500 transform {}", if current_index() == i { "opacity-100 scale-100" } else { "opacity-50 scale-75 blur-sm" }),
                        div { class: format!("{} p-8 rounded-lg shadow-lg text-center max-w-sm border",
                                        if dark_mode() == Theme::Dark { "border-gray-700 bg-gray-800" } else { "bg-white border-gray-300" }),
                            StarRating { star_images: testimonial.star_images.clone() }
                            blockquote { class: "text-lg font-semibold", "{testimonial.quote}" }
                            AuthorInfo {
                                author_image: testimonial.author_image,
                                author_name: testimonial.author_name,
                                author_title: testimonial.author_title,
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
