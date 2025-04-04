use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AuthorProps {
    author_image: Asset,
    author_name: &'static str,
    author_title: &'static str,
    company_logo: Asset,
}

#[component]
pub fn AuthorInfo(props: AuthorProps) -> Element {
    rsx! {
        div { class: "flex items-center justify-center mt-4 space-x-4",
            img { src: "{props.author_image}", class: "w-10 h-10 rounded-full" }
            div { class: "text-left",
                p { class: "text-sm font-semibold", "{props.author_name}" }
                p { class: "text-xs text-gray-500", "{props.author_title}" }
            }
            img { src: "{props.company_logo}", class: "w-12 h-6" }
        }
    }
}
