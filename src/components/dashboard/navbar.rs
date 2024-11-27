use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::server::auth::controller::about_me;
use crate::server::auth::model::User;
use crate::theme::Theme;
use crate::theme::ThemeToggle;
use dioxus::prelude::*;
use gloo_storage::Storage;
use gloo_storage::{LocalStorage, SessionStorage};

#[component]
pub fn Navbar() -> Element {
    let mut show_dropdown = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    let navigator = use_navigator();

    let mut user_data = use_signal(|| None::<User>);

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(res) => {
                        user_data.set(Some(res.data.user));
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });

    let handle_logout = move |e: Event<MouseData>| {
        e.stop_propagation();
        loading.set(false);

        SessionStorage::clear();
        LocalStorage::clear();
        navigator.push("/login");
    };

    let handle_profile = move |e: Event<MouseData>| {
        e.stop_propagation();
        loading.set(false);

        if user_data().is_some() {
            navigator.push(format!("/dashboard/profile/{}", user_data().unwrap().id));
        }
    };

    rsx! {
        div { class: format!("flex justify-between items-center mb-4 border-b shadow-sm p-2 {}", if dark_mode { "dark:border-gray-700" } else { "" }),
            h1 { class: "text-2xl font-semibold", "Dashboard" }

            div { class: "flex items-center space-x-4",
                ThemeToggle {}

                div { class: "relative",
                    button {
                        class: format!("p-2 rounded-full flex items-center justify-center {}", if dark_mode { "bg-gray-700" } else { "bg-gray-200" }),
                        onclick: move |_| show_dropdown.set(!show_dropdown()),
                        img {
                            src: "https://rustacean.net/assets/rustacean-orig-noshadow.svg",
                            alt: "User profile image",
                            class: "w-8 h-8 rounded-full"
                        }
                    }
                    if show_dropdown() {
                        div { class: format!("absolute right-0 mt-2 w-48 shadow-lg rounded-lg {}", if dark_mode { "bg-gray-800" } else { "bg-white" }),
                            button {
                                class: format!("w-full text-left px-4 py-2 hover:bg-gray-100 {}", if dark_mode { "hover:bg-gray-700" } else { "" }),
                                onclick: handle_profile,
                                "Profile"
                            }
                            button {
                                class: "w-full text-left px-4 py-2 hover:bg-gray-100",
                                onclick: handle_logout,
                                if loading() {
                                    Spinner {
                                        aria_label: "Loading spinner".to_string(),
                                        size: SpinnerSize::Md,
                                        dark_mode: true,
                                    }
                                    span { "logging out..." }
                                } else {
                                    span { "Log Out" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
