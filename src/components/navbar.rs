pub(crate) mod btns;
pub(crate) mod links;

use crate::components::common::logo::Logo;
use crate::components::navbar::btns::AuthButtons;
use crate::components::navbar::links::NavLinks;
use crate::i18n::I18nToggle;
use crate::router::Route;
use crate::theme::ThemeToggle;

use dioxus::prelude::*;

#[component]
fn NavBar(show_items: bool) -> Element {
    let mut is_menu_open = use_signal(|| false);

    let toggle_menu = move |_| {
        is_menu_open.set(!is_menu_open());
    };

    rsx! {
        nav {
            class: "fixed top-0 w-full z-50 flex items-center justify-between px-8 py-4 transition-colors duration-300",
            Link {
                to: "/",
                Logo {}
            }

            div {
                class: "items-center justify-between px-8 py-4 shadow-md hidden md:flex rounded-lg dark:bg-white dark:text-black bg-gray-900 text-white",
                NavLinks {show_items},
                AuthButtons { is_vertical: false }
            }

            div {
                class: "flex space-x-4",
                ThemeToggle {}
                I18nToggle {}
            }

            button {
                class: format!("text-3xl md:hidden transform duration-300 dark:text-white text-black {}",
                    if is_menu_open() { "rotate-90" } else { "rotate-0" },
                ),

                onclick: toggle_menu,
                if is_menu_open() { "✕" } else { "☰" }
            }

            div {
                class: format!(
                    "fixed top-0 left-0 w-2/5 md:w-auto h-auto p-4 z-50 md:hidden transition-transform transform duration-500 ease-in-out dark:bg-gray-900 dark:text-white bg-white text-black {}",
                    if is_menu_open() { "translate-x-0 opacity-100" } else { "-translate-x-full opacity-0" },
                ),
                NavLinks {show_items}
                AuthButtons { is_vertical: true }
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
pub fn HomeNavBar() -> Element {
    rsx! {
        NavBar {show_items: true}
    }
}

#[component]
pub fn LoginNavBar() -> Element {
    rsx! {
        NavBar {show_items: false}
    }
}
