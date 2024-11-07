use crate::pages::dashboard::toggle_theme;
use dioxus::prelude::*;

#[component]
pub fn Navbar(dark_mode: bool) -> Element {
    let mut show_dropdown = use_signal(|| false);

    rsx! {
        div { class: format!("flex justify-between items-center mb-4 border-b shadow-sm p-2 {}", if dark_mode { "dark:border-gray-700" } else { "" }),
            h1 { class: "text-2xl font-semibold", "User Dashboard" }

            div { class: "flex items-center space-x-4",
                button {
                    onclick: |_| toggle_theme(),
                    class: "p-2 rounded-full text-lg",
                    if dark_mode { "🌙" } else { "🌞" }
                }

                div { class: "relative",
                    button {
                        class: format!("p-2 rounded-full flex items-center justify-center {}", if dark_mode { "bg-gray-700" } else { "bg-gray-200" }),
                        onclick: move |_| show_dropdown.set(!show_dropdown()),
                        img {
                            src: "./features.png",
                            alt: "User profile image",
                            class: "w-8 h-8 rounded-full"
                        }
                    }
                    if show_dropdown() {
                        div { class: format!("absolute right-0 mt-2 w-48 shadow-lg rounded-lg {}", if dark_mode { "bg-gray-800" } else { "bg-white" }),
                            button { class: format!("w-full text-left px-4 py-2 hover:bg-gray-100 {}", if dark_mode { "dark:hover:bg-gray-700" } else { "" }), "Home" }
                            button { class: format!("w-full text-left px-4 py-2 hover:bg-gray-100 {}", if dark_mode { "dark:hover:bg-gray-700" } else { "" }), "Profile" }
                            button { class: format!("w-full text-left px-4 py-2 hover:bg-gray-100 {}", if dark_mode { "dark:hover:bg-gray-700" } else { "" }), "Logout" }
                        }
                    }
                }
            }
        }
    }
}
