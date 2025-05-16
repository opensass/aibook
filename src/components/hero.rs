use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Hero() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        section {
            class: "min-h-screen flex flex-col items-center justify-center transition-colors duration-300 px-6 dark:bg-gray-900 dark:text-white bg-white text-black",
            div {
                class: "text-center space-y-6",
                p {
                    class: "text-lg uppercase tracking-widest text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-red-600 animate-glow",
                    "{i18n().t(\"hero.new\")}"
                }
                h1 {
                    class: "text-5xl md:text-7xl font-bold",
                    "{i18n().t(\"hero.title\")}"
                },
                p {
                    class: "text-xl md:text-2xl",
                    "{i18n().t(\"hero.subtitle\")}"
                },
                div {
                    class: "flex justify-center space-x-4",
                    Link {
                        to: "/login",
                        class: "bg-gray-500 text-white py-2 px-4 rounded-lg shadow hover:bg-gray-600 focus:outline-none",
                        "{i18n().t(\"hero.cta\")}"
                    }
                }
                div {
                    class: "pt-8 max-w-3xl mx-auto text-center bg-clip-text bg-gradient-to-r from-purple-200 to-red-800 animate-glow",
                    p {
                        class: "text-lg md:text-xl text-gray-600 dark:text-gray-400",
                        "{i18n().t(\"hero.description\")}"
                    }
                }
            }
        }
    }
}
