use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[component]
pub fn InputField(
    label: &'static str,
    value: Signal<String>,
    is_valid: Signal<bool>,
    validate: fn(&str) -> bool,
    required: bool,
) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;

    let handle_input = move |e: Event<FormData>| {
        let input_value = e.value().clone();
        value.set(input_value.clone());
        is_valid.set(validate(&input_value));
    };

    rsx! {
        div {
            label {
                class: format!("block text-sm font-medium {}", if dark_mode { "text-gray-300" } else { "text-gray-700" }),
                "{label}"
            }
            input {
                class: format!(
                    "mt-1 block w-full p-2 border rounded-md shadow-sm {} {}",
                    if dark_mode { "bg-gray-900" } else { "" },
                    if is_valid() { "border-gray-300" } else { "border-red-500"
                }),
                value: "{value}",
                oninput: handle_input,
                required: required
            }
            if !is_valid() {
                p { class: "text-red-500 text-sm mt-1", "Invalid input" }
            }
        }
    }
}
