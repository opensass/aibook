use crate::components::footer::links::FooterLink;
use dioxus::prelude::*;

#[component]
pub fn Support() -> Element {
    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            h5 { class: "text-lg font-semibold mb-4", "Support" }
            ul {
                class: "space-y-2",
                FooterLink { href: "/forget-password", text: "Forget Password" },
                FooterLink { href: "/faq", text: "FAQs" },
                FooterLink { href: "/contact", text: "Contact" },
            }
        }
    }
}
