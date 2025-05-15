use crate::components::dashboard::books::create::CreateBookPanel;
use crate::components::dashboard::books::list::BooksPanel;
use crate::components::dashboard::books::read::ReadBookPanel;
use crate::components::dashboard::chat::ChatPanelPage;
use crate::components::dashboard::navbar::Navbar;
use crate::components::dashboard::profile::ProfilePagePanel;
use crate::components::dashboard::sidebar::Sidebar;
use crate::components::dashboard::sidebar::Tab;
use crate::server::auth::controller::about_me;
use dioxus::prelude::*;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;

#[component]
pub fn Dashboard() -> Element {
    let active_tab = use_signal(|| Tab::Books);
    let mut user_token = use_signal(|| "".to_string());
    let navigator = use_navigator();
    let current_tab = match active_tab() {
        Tab::Books => rsx! { BooksPanel { user_token } },
        Tab::CreateBook => rsx! { CreateBookPanel { user_token } },
        Tab::ReadBook => rsx! { ReadBookPanel { book_id: "" } },
        Tab::EditProfile => rsx! { ProfilePagePanel {} },
        Tab::Chat => rsx! { ChatPanelPage { user_token, book_id: ""} },
    };

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
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
        div { class: "min-h-screen flex dark:bg-gray-900 text-white bg-white text-gray-900",
            Sidebar { navigate: false, active_tab: active_tab.clone() }

            div { class: "flex-1 p-4 md:p-8",
                Navbar {}

                div { class: "p-4 shadow rounded-lg dark:bg-gray-800 bg-white",
                    {current_tab}
                }
            }
        }
    }
}
