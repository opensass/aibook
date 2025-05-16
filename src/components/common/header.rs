use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    title: String,
    subtitle: String,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        div { class: "mb-20 justify-center text-center",
            h2 { class: "text-4xl md:text-5xl font-bold leading-tight mt-4 mb-6 dark:bg-gray-900 dark:text-white bg-white text-gray-900",
                "{props.title}"
            },
            p { class: "text-lg leading-relaxed mb-8 dark:bg-gray-900 dark:text-gray-400 bg-white text-gray-800",
                "{props.subtitle}"
            }
        }
    }
}
