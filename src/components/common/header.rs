use crate::theme::Theme;
use crate::theme::THEME;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    title: &'static str,
    subtitle: &'static str,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    let dark_mode = *THEME.read();
    rsx! {
        div { class: "max-w-[600px] mb-20 justify-center text-center",
            h2 { class: format!("text-4xl md:text-5xl font-bold leading-tight mt-4 mb-6 {}", if dark_mode == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),
                "{props.title}"
            },
            p { class: format!("text-lg leading-relaxed mb-8 {}", if dark_mode == Theme::Dark { "bg-gray-900 text-gray-400" } else { "bg-white text-gray-800" }),
                "{props.subtitle}"
            }
        }
    }
}
