use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ItemProps {
    icon: &'static str,
    title: String,
    description: String,
}

#[component]
pub fn FeatureItem(props: ItemProps) -> Element {
    rsx! {
        div {
            class: format!(
                "flex flex-col items-center p-6 rounded-lg transition-all duration-300 border border-gray-300 hover:shadow-lg
                shadow-md {} dark:bg-gray-800 dark:hover:bg-gray-700 bg-white hover:bg-gray-100",
                "transform hover:-translate-y-1 hover:shadow-lg"
            ),
            i {
                class: format!("w-12 h-12 mb-4 transform transition-transform duration-300 hover:scale-110 {}", props.icon),
            }
            h3 {
                class: "text-lg font-semibold transition-colors duration-300 dark:text-white text-gray-800",
                "{props.title}"
            }
            p {
                class: "text-sm text-center transition-colors duration-300 dark:text-gray-400 text-gray-600",
                "{props.description}"
            }
        }
    }
}
