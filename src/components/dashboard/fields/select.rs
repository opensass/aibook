use dioxus::prelude::*;

#[component]
pub fn SelectField(
    label: &'static str,
    options: Vec<&'static str>,
    selected: Signal<String>,
) -> Element {
    rsx! {
        div {
            label { class: "block text-sm font-medium dark:text-gray-300 text-gray-700", "{label}" }
            select {
                class: "mt-1 block w-full p-2 border rounded-md shadow-sm dark:bg-gray-900 dark:border-gray-700 border-gray-300",
                value: "{selected}",
                oninput: move |e| selected.set(e.value().clone()),
                for option in options {
                    option { value: "{option}", "{option}" }
                }
            }
        }
    }
}
