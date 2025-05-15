use crate::components::common::logo::Logo;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Tab {
    Books,
    Chat,
    CreateBook,
    ReadBook,
    EditProfile,
}

#[component]
pub fn Sidebar(active_tab: Signal<Tab>, navigate: bool) -> Element {
    let navigator = use_navigator();

    let tab_style = |tab: Tab| -> String {
        if active_tab() == tab {
            "w-full p-2 flex items-center space-x-2 rounded bg-blue-500 text-white dark:bg-blue-600"
                .to_string()
        } else {
            "w-full p-2 flex items-center space-x-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 dark:text-gray-400 text-gray-600".to_string()
        }
    };

    rsx! {
        div { class: "fixed bottom-0 w-full md:static md:w-64 p-4 space-y-4 md:min-h-screen flex md:flex-col items-center md:items-start dark:bg-gray-900 bg-gray-200",
            Link {
                to: "/dashboard",
                class: "hidden md:inline",
                Logo {}
            }

            div { class: tab_style(Tab::Books),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::Books);
                },
                i { class: "fas fa-folder-open text-2xl" },
                span { class: "hidden md:inline", "Books" }
            }

            div { class: tab_style(Tab::Chat),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::Chat);
                },
                i { class: "fas fa-comment text-2xl" },
                span { class: "hidden md:inline", "Chat" }
            }

            div { class: tab_style(Tab::CreateBook),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::CreateBook);
                },
                i { class: "fas fa-file-alt text-2xl" },
                span { class: "hidden md:inline", "Generate" }
            }

            div { class: tab_style(Tab::ReadBook),
                onclick: move |_| active_tab.set(Tab::ReadBook),
                i { class: "fas fa-address-book text-2xl" },
                span { class: "hidden md:inline", "Read Book" }
            }

            div { class: tab_style(Tab::EditProfile),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::EditProfile);
                },
                i { class: "fas fa-user-edit text-2xl" },
                span { class: "hidden md:inline", "Profile" }
            }
        }
    }
}
