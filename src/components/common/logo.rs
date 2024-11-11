use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "flex items-center",
            img {
                src: "./logo.webp",
                alt: "AI Book Logo",
                class: "w-24 h-24 object-contain"
            }
        }
    }
}
