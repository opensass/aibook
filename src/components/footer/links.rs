use crate::components::footer::icon::SocialIcon;
use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn ContactLink(label: String, href: String, text: String) -> Element {
    rsx! {
        li {
            p { class: "font-semibold text-gray-500", "{label}" }
            a { href: "{href}", class: "text-sm hover:text-white transition-colors", "{text}" }
        }
    }
}

#[component]
pub fn QuickLinks() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            h5 { class: "text-lg font-semibold mb-4", "{i18n().t(\"footer.links.title\")}" }
            ul {
                class: "space-y-2",
                FooterLink { href: "/", text: i18n().t("footer.links.home") },
                FooterLink { href: "/project", text: i18n().t("footer.links.project") },
                FooterLink { href: "/blog", text: i18n().t("footer.links.blog") },
                FooterLink { href: "/team", text: i18n().t("footer.links.team") },
            }
        }
    }
}

#[component]
pub fn FooterLink(href: &'static str, text: String) -> Element {
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
            SocialIcon { href: "https://www.linkedin.com/company/opensass", icon: rsx! {
                i { class: "fab fa-linkedin-in text-2xl" }
            }},
            SocialIcon { href: "https://www.x.com/opensassorg", icon: rsx! {
                i { class: "fab fa-x-twitter text-2xl" }
            }},
            SocialIcon { href: "https://www.github.com/opensass", icon: rsx! {
                i { class: "fab fa-github text-2xl" }
            }},
        }
    }
}
