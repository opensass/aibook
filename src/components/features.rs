pub(crate) mod grid;
pub(crate) mod item;

use crate::components::common::header::Header;
use crate::components::features::grid::Grid;
use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[derive(Props, Clone, PartialEq)]
struct Feature {
    icon: &'static str,
    title: String,
    description: String,
}

#[component]
pub fn Features() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    let features = vec![
        Feature {
            icon: "text-2xl fas fa-compass",
            title: i18n().t("features.language_support.title"),
            description: i18n().t("features.language_support.description"),
        },
        Feature {
            icon: "text-2xl fab fa-google",
            title: i18n().t("features.gemini.title"),
            description: i18n().t("features.gemini.description"),
        },
        Feature {
            icon: "text-2xl fab fa-rust",
            title: i18n().t("features.rust_security.title"),
            description: i18n().t("features.rust_security.description"),
        },
        Feature {
            icon: "text-2xl fas fa-star",
            title: i18n().t("features.real_time.title"),
            description: i18n().t("features.real_time.description"),
        },
        Feature {
            icon: "text-2xl fas fa-chart-bar",
            title: i18n().t("features.analytics.title"),
            description: i18n().t("features.analytics.description"),
        },
        Feature {
            icon: "text-2xl fas fa-keyboard",
            title: i18n().t("features.dev_friendly.title"),
            description: i18n().t("features.dev_friendly.description"),
        },
    ];

    rsx! {
        section {
            id: "features",
            class: "py-20 px-8 md:px-4 font-roboto flex min-h-screen justify-center dark:bg-gray-900 dark:text-white bg-white text-gray-900",

            div { class: "max-w-[1000px] mx-auto text-center",

                div { class: "relative mb-12",
                    img {
                        src: asset!("/assets/features.webp"),
                        alt: "AIBook Icon",
                        class: "w-24 h-24 mx-auto animate-bounce"
                    }
                    Header {
                        title: {i18n().t("features.header.title")},
                        subtitle: {i18n().t("features.header.subtitle")}
                    }
                }

                Grid { features: features }
            }
        }
    }
}
