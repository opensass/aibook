use crate::components::footer::icon::SocialIcon;
use dioxus::prelude::*;

#[component]
pub fn ContactLink(label: &'static str, href: &'static str, text: &'static str) -> Element {
    rsx! {
        li {
            p { class: "font-semibold text-gray-500", "{label}" }
            a { href: "{href}", class: "text-sm hover:text-white transition-colors", "{text}" }
        }
    }
}

#[component]
pub fn QuickLinks() -> Element {
    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            h5 { class: "text-lg font-semibold mb-4", "Links" }
            ul {
                class: "space-y-2",
                FooterLink { href: "/", text: "Home" },
                FooterLink { href: "/project", text: "Project" },
                FooterLink { href: "/blog", text: "Blog" },
                FooterLink { href: "/team", text: "Our Team" },
            }
        }
    }
}

#[component]
pub fn FooterLink(href: &'static str, text: &'static str) -> Element {
    rsx! {
        li {
            Link { to: "{href}", class: "text-sm text-gray-400 hover:text-white transition-colors", "{text}" }
        }
    }
}

#[component]
pub fn SocialLinks() -> Element {
    rsx! {
        ul {
            class: "flex space-x-4",
            SocialIcon { href: "#" },
            SocialIcon { href: "#" },
        }
    }
}
