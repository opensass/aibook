use crate::router::Route;
use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthButtonsProps {
    is_vertical: bool,
}

#[component]
pub fn AuthButtons(props: AuthButtonsProps) -> Element {
    let dark_mode = *THEME.read();
    let button_class = if props.is_vertical {
        "flex flex-col gap-4"
    } else {
        "flex flex-row gap-4"
    };

    rsx! {
        div { class: "{button_class}",
            Link {
                to: Route::Register {},
                class: format!(
                    "border px-5 py-2 text-lg hover:bg-gray-100 {}",

                if dark_mode == Theme::Dark { "border-gray-700" } else { "border-gray-300" }
                ),
                "Register"
            }
            Link {
                to: Route::Login {},
                class: "bg-gray-600 text-white px-5 py-2 text-lg rounded hover:bg-gray-700",
                "Login"
            }
        }
    }
}
