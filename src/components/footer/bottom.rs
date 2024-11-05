use crate::components::footer::links::SocialLinks;
use dioxus::prelude::*;

#[component]
pub fn Bottom() -> Element {
    rsx! {
        div {
            class: "border-t border-gray-700 mt-10 pt-6",
            div {
                class: "container mx-auto px-6 lg:px-16 flex flex-col lg:flex-row items-center justify-between space-y-4 lg:space-y-0",
                div {
                    class: "text-sm text-gray-500",
                    "© 2024. Designed by ",
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
