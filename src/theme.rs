use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
pub fn ThemeToggle() -> Element {
    let theme_ctx = use_theme();

    let onclick = {
        move |_| {
            let new_theme = match (theme_ctx.theme)() {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
                _ => Theme::Light,
            };
            theme_ctx.set_theme.call(new_theme);
        }
    };

    rsx! {
        div { class: "flex items-center justify-center",
            button {
                onclick: onclick,
                class: "relative w-[50px] h-[26px] rounded-full bg-gray-300 dark:bg-gray-800 p-1 flex items-center justify-between transition-colors duration-300",
                span {
                    class: "absolute top-[2px] left-[2px] w-[22px] h-[22px] rounded-full bg-white transition-transform duration-300 transform translate-x-0 dark:translate-x-[24px]"
                }
                span {
                    class: "absolute inset-0 flex items-center justify-between px-2 text-xs z-0",
                    i {
                        class: "fas fa-moon text-yellow-400 dark:opacity-100 opacity-0 transition-opacity duration-300"
                    }
                    i {
                        class: "fas fa-sun text-yellow-600 dark:opacity-0 opacity-100 transition-opacity duration-300"
                    }
                }
            }
        }
    }
}
