use crate::components::footer::links::ContactLink;
use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn Contact() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();

    rsx! {
        div {
            class: "mb-6 lg:mb-0",
            h5 { class: "text-lg font-semibold mb-4", "{i18n().t(\"footer.contact.title\")}" }
            ul {
                class: "space-y-2 text-gray-400",
                ContactLink { label: i18n().t("footer.contact.address_label"), href: "#", text: i18n().t("footer.contact.address_text") },
                ContactLink { label: i18n().t("footer.contact.email_label"), href: "mailto:oss@opensass.org", text: "oss@opensass.org" },
            }
        }
    }
}
