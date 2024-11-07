use crate::components::common::logo::Logo;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_regular_icons::{
    FaAddressBook, FaFileLines, FaFolderOpen, FaPenToSquare,
};
use dioxus_free_icons::Icon;

#[derive(PartialEq, Clone)]
pub enum Tab {
    Books,
    CreateBook,
    ReadBook,
    EditProfile,
}

#[component]
pub fn Sidebar(active_tab: Signal<Tab>, navigate: bool) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    let navigator = use_navigator();

    let tab_style = |tab: Tab| -> String {
        if active_tab() == tab {
            format!(
                "w-full p-2 flex items-center space-x-2 rounded bg-blue-500 text-white {}",
                if dark_mode { "dark:bg-blue-600" } else { "" }
            )
        } else {
            format!(
                "w-full p-2 flex items-center space-x-2 rounded hover:bg-gray-100 {}",
                if dark_mode {
                    "dark:hover:bg-gray-700 text-gray-400"
                } else {
                    "text-gray-600"
                }
            )
        }
    };

    rsx! {
        div { class: format!("fixed bottom-0 w-full md:static md:w-64 p-4 space-y-4 md:min-h-screen flex md:flex-col items-center md:items-start {}",
                              if dark_mode { "bg-gray-900" } else { "bg-gray-200" }),
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
                Icon {
                    width: 30,
                    height: 30,
                    icon: FaFolderOpen,
                },
                span { class: "hidden md:inline", "Books" }
            }
            div { class: tab_style(Tab::CreateBook),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::CreateBook);
                },
                Icon {
                    width: 30,
                    height: 30,
                    icon: FaFileLines,
                },
                span { class: "hidden md:inline", "Create Book" }
            }
            div { class: tab_style(Tab::ReadBook),
                onclick: move |_| active_tab.set(Tab::ReadBook),
                Icon {
                    width: 30,
                    height: 30,
                    icon: FaAddressBook,
                },
                span { class: "hidden md:inline", "Read Book" }
            }
            div { class: tab_style(Tab::EditProfile),
                onclick: move |_| {
                    if navigate {
                        navigator.push("/dashboard");
                    }
                    active_tab.set(Tab::EditProfile);
                },
                Icon {
                    width: 30,
                    height: 30,
                    icon: FaPenToSquare,
                },
                span { class: "hidden md:inline", "Edit Profile" }
            }
        }
    }
}
