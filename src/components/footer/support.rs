use crate::components::footer::links::FooterLink;
use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Support() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            h5 { class: "text-lg font-semibold mb-4", "{i18n().t(\"footer.support.title\")}" }
            ul {
                class: "space-y-2",
                FooterLink { href: "/forget-password", text: i18n().t("footer.support.forget_password") },
                FooterLink { href: "/faq", text: i18n().t("footer.support.faq") },
                FooterLink { href: "/contact", text: i18n().t("footer.support.contact") },
            }
        }
    }
}
