use crate::components::features::Features;
use crate::components::hero::Hero;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "font-sans",
            Hero {}
            Features {}
        }
    }
}
