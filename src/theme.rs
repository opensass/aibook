use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Theme {
    Light,
    Dark,
}

pub static THEME: GlobalSignal<Theme> = GlobalSignal::new(|| Theme::Dark);

#[component]
pub fn ThemeToggle() -> Element {
    use_effect(|| {
        *THEME.write() = Theme::Dark;
    });

    let mut theme = use_signal(|| Theme::Dark);

    let toggle_theme = move |_| {
        let new_theme = if theme() == Theme::Light {
            Theme::Dark
        } else {
            Theme::Light
        };
        theme.set(new_theme);
        *THEME.write() = new_theme;
    };

    rsx! {
        button {
            onclick: toggle_theme,
            class: "p-2 rounded-lg text-sm font-medium transition-colors duration-300 bg-gray-700 text-white hover:bg-gray-600",
            if theme() == Theme::Dark { " 🌜 Dark " } else { "🌞 Light" }
        }
    }
}
