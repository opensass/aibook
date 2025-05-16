use dioxus::prelude::*;
use i18nrs::dioxus::I18nContext;

#[component]
pub fn I18nToggle() -> Element {
    let I18nContext { set_language, .. } = use_context::<I18nContext>();
    let mut language_state = use_signal(|| "en".to_string());

    rsx! {
        select {
            class: "border rounded-md p-2",
            onchange: move |event| {
                let value = event.value();
                language_state.set(value.clone());
                set_language.call(value);
            },
            option { value: "en", "🇺🇸 English" }
            option { value: "fr", "🇫🇷 French" }
            option { value: "es", "🇪🇸 Spanish" }
            option { value: "ar", "🇸🇦 Arabic" }
        }
    }
}
