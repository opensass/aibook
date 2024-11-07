use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[component]
pub fn SelectField(
    label: &'static str,
    options: Vec<&'static str>,
    selected: Signal<String>,
) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    rsx! {
        div {
            label { class: format!("block text-sm font-medium {}", if dark_mode { "text-gray-300" } else { "text-gray-700" }), "{label}" }
            select {
                class: format!("mt-1 block w-full p-2 border rounded-md shadow-sm {}", if dark_mode { "bg-gray-900 border-gray-700" } else { "border-gray-300" }),
                value: "{selected}",
                oninput: move |e| selected.set(e.value().clone()),
                for option in options {
                    option { value: "{option}", "{option}" }
                }
            }
        }
    }
}
