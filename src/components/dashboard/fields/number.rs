use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[component]
pub fn NumberField(label: &'static str, value: Signal<u64>, required: bool) -> Element {
    let dark_mode = *THEME.read() == Theme::Dark;
    rsx! {
        div {
            label { class: format!("block text-sm font-medium {}", if dark_mode { "text-gray-300" } else { "text-gray-700" }), "{label}" }
            input {
                r#type: "number",
                class: format!("mt-1 block w-full p-2 border rounded-md shadow-sm {}", if dark_mode { "bg-gray-900 border-gray-700" } else { "border-gray-300" }),
                value: "{value}",
                oninput: move |e| if let Ok(val) = e.value().parse() { value.set(val); },
                required: required
            }
        }
    }
}
