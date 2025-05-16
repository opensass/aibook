use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Logo() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            div {
                class: "flex items-center space-x-2 mb-4",
                img { src: asset!("/assets/logo.webp"), alt: "Logo", class: "h-24" },
            }
            p { class: "text-sm text-gray-400", "{i18n().t(\"footer.logo.description\")}" }
        }
    }
}
