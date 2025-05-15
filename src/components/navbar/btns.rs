use crate::router::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthButtonsProps {
    is_vertical: bool,
}

#[component]
pub fn AuthButtons(props: AuthButtonsProps) -> Element {
    let button_class = if props.is_vertical {
        "flex flex-col gap-4"
    } else {
        "flex flex-row gap-4"
    };

    rsx! {
        div { class: "{button_class}",
            Link {
                to: Route::Register {},
                class: "border px-5 py-2 text-lg hover:bg-gray-100 dark:border-gray-700 border-gray-300",
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
