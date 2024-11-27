pub(crate) mod edit;
pub(crate) mod view;

use crate::components::dashboard::profile::edit::ProfileForm;
use crate::components::dashboard::profile::view::ProfileDetails;
use crate::server::auth::controller::about_me;
use crate::server::auth::model::User;
use crate::theme::Theme;

use dioxus::prelude::*;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;

#[component]
pub fn ProfilePagePanel() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let dark_mode = theme() == Theme::Dark;
    let mut user_token = use_signal(|| "".to_string());
    let mut user_data = use_signal(|| None::<User>);
    let mut edit_mode = use_signal(|| false);
    let navigator = use_navigator();

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(res) => {
                        user_data.set(Some(res.data.user));
                        user_token.set(token.clone());
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });

    rsx! {
        div { class: format!("p-4 {}", if dark_mode { "bg-gray-800 text-white" } else { "bg-white text-gray-900" }),
            h2 { class: "text-xl font-semibold mb-4", "Profile" }
                div { class: "container mx-auto p-4",
                    div { class: "flex items-center justify-between",
                        button {
                            class: format!("py-2 px-4 rounded {}", if dark_mode { "bg-blue-600" } else { "bg-blue-500 text-white" }),
                            onclick: move |_| edit_mode.set(!edit_mode()),
                            if edit_mode() { "Cancel" } else { "Edit" }
                        }
                    }

                    div { class: format!("mt-6 space-y-4 bg-white shadow-md p-4 rounded-md {}", if dark_mode { "bg-gray-800" } else { "bg-white" }),
                        match user_data.as_ref() {
                            Some(user) => rsx! {
                                if edit_mode() {
                                    ProfileForm { user: user.clone(), dark_mode, user_token }
                                } else {
                                    ProfileDetails { user: user.clone(), dark_mode, user_token }
                                }
                            },
                            None => rsx!(p { "Loading..." })
                        }
                    }
                }
        }
    }
}
