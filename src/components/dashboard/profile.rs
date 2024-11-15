use crate::theme::Theme;
use dioxus::prelude::*;

#[component]
pub fn EditProfilePanel() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    rsx! {
        div { class: format!("p-4 {}", if dark_mode { "bg-gray-800 text-white" } else { "bg-white text-gray-900" }),
            h2 { class: "text-xl font-semibold mb-4", "Edit Profile" }
            p { "TODO" }
        }
    }
}
