use dioxus::prelude::*;

#[component]
pub fn SocialIcon(href: &'static str, icon: Element) -> Element {
    rsx! {
        li {
            a {
                href: "{href}",
                target: "_black",
                class: "p-2 bg-gray-800 rounded-full hover:text-black transition-colors",
                {icon}
            }
        }
    }
}
