use dioxus::prelude::*;

#[component]
pub fn Logo() -> Element {
    rsx! {
        div { class: "flex items-center",
            img {
                src: "https://aibook-8syx.onrender.com/logo.webp",
                alt: "AI Book Logo",
                class: "w-24 h-24 object-contain"
            }
        }
    }
}
