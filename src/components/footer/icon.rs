use dioxus::prelude::*;

#[component]
pub fn SocialIcon(href: &'static str) -> Element {
    rsx! {
        li {
            a {
                href: "{href}",
                class: "p-2 bg-gray-800 rounded-full hover:text-black transition-colors",
                svg {
                    class: "w-6 h-6",
                    fill: "none",
                    stroke: "currentColor",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M5 10l7-7m0 0l7 7m-7-7v18"
                    }
                }
            }
        }
    }
}
