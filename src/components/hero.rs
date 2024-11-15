use crate::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    let dark_mode = use_context::<Signal<Theme>>();

    rsx! {
        section {
            class: format!(
                "min-h-screen flex flex-col items-center justify-center transition-colors duration-300 px-6 {}",
                if dark_mode() == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-black" }
            ),
            div {
                class: "text-center space-y-6",
                p {
                    class: "text-lg uppercase tracking-widest text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-red-600 animate-glow",
                    "New"
                }
                h1 {
                    class: "text-5xl md:text-7xl font-bold",
                    "Effortless Content Creation"
                },
                p {
                    class: "text-xl md:text-2xl",
                    "Empower your creativity with AIbook, the ultimate platform to generate high-quality content in seconds."
                },
                div {
                    class: "flex justify-center space-x-4",
                    button {
                        class: "bg-gray-500 text-white py-2 px-4 rounded-lg shadow hover:bg-gray-600 focus:outline-none",
                        "Get Started"
                    }
                }
                div {
                    class: "pt-8 max-w-3xl mx-auto text-center bg-clip-text bg-gradient-to-r from-purple-200 to-red-800 animate-glow",
                    p {
                        class: "text-lg md:text-xl text-gray-600 dark:text-gray-400",
                        "Create books and social posts effortlessly with AIbook."
                    }
                }
            }
        }
    }
}
