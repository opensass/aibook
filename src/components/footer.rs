pub(crate) mod bottom;
pub(crate) mod contact;
pub(crate) mod icon;
pub(crate) mod links;
pub(crate) mod logo;
pub(crate) mod support;

use crate::components::footer::bottom::Bottom;
use crate::components::footer::contact::Contact;
use crate::components::footer::links::QuickLinks;
use crate::components::footer::logo::Logo;
use crate::components::footer::support::Support;
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "bg-gray-800 text-white py-10",
            div {
                class: "container mx-auto px-6 lg:px-16 grid gap-8 lg:grid-cols-4 grid-cols-1",
                Logo {},
                Contact {},
                Support {},
                QuickLinks {},
            },
            Bottom {},
        }
    }
}
