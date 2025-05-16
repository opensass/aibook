use crate::components::footer::links::SocialLinks;
use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Bottom() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        div {
            class: "border-t border-gray-700 mt-10 pt-6",
            div {
                class: "container mx-auto px-6 lg:px-16 flex flex-col lg:flex-row items-center justify-between space-y-4 lg:space-y-0",
                div {
                    class: "text-sm text-gray-500",
                    "© 2025. ",
                    "{i18n().t(\"footer.bottom.designed_by\")}",
                    a {
                        href: "https://github.com/opensass",
                        class: "text-white hover:text-gray-400 transition-colors",
                        "OpenSASS"
                    }
                },
                SocialLinks {},
            }
        }
    }
}
