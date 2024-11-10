pub(crate) mod panel;
pub(crate) mod sidebar;

use crate::components::dashboard::chat::panel::ChatPanel;
use crate::components::dashboard::chat::sidebar::ConversationsSidebar;
use crate::components::spinner::{Spinner, SpinnerSize};
use crate::server::conversation::controller::get_conversations;
use crate::server::conversation::model::Conversation;
use crate::server::conversation::request::GetConversationsRequest;
use bson::oid::ObjectId;
use chrono::Utc;
use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CachedConversationsData {
    pub book_id: String,
    pub conversations: Vec<Conversation>,
    pub timestamp: i64,
}

pub const CONVERSATIONS_CACHE_KEY: &str = "conversations_cache";
pub const CONVERSATIONS_CACHE_TIMEOUT: i64 = 2 * 60 * 60;

#[component]
pub fn ChatPanelPage(user_token: Signal<String>, book_id: String) -> Element {
    let selected_conversation = use_signal(ObjectId::new);
    let book_id_clone = book_id.clone();
    let mut conversations = use_signal(Vec::<Conversation>::new);
    let mut loading = use_signal(|| true);
    let mut is_sidebar_visible = use_signal(|| false);

    use_effect(move || {
        let book_id = book_id.clone();
        spawn(async move {
            let now = Utc::now().timestamp();

            if let Ok(cached_data) =
                LocalStorage::get::<CachedConversationsData>(CONVERSATIONS_CACHE_KEY)
            {
                if cached_data.book_id == book_id
                    && now - cached_data.timestamp < CONVERSATIONS_CACHE_TIMEOUT
                {
                    loading.set(false);
                    conversations.set(cached_data.conversations.clone());
                    return;
                }
            }

            if let Ok(response) = get_conversations(GetConversationsRequest {
                token: user_token(),
                book_id: book_id.clone(),
            })
            .await
            {
                loading.set(false);
                conversations.set(response.data.clone());

                let cached_data = CachedConversationsData {
                    book_id: book_id,
                    conversations: response.data.clone(),
                    timestamp: now,
                };
                let _ = LocalStorage::set(CONVERSATIONS_CACHE_KEY, &cached_data);
            } else {
                loading.set(true);
            }
        });
    });

    rsx! {
        div {
            class: "flex min-h-screen",
            div {
                class: "border-r border-gray-600 bg-gray-900 min-h-screen hidden md:block",
                if loading() {
                    Spinner {
                        aria_label: "Loading conversations...".to_string(),
                        size: SpinnerSize::Md,
                        dark_mode: true,
                    }
                } else {
                    ConversationsSidebar {
                        conversations,
                        selected_conversation,
                        token: user_token(),
                        book_id: book_id_clone.clone()
                    }
                }
            }

            div {
                class: "flex-1 flex flex-col h-full dark:bg-gray-800",
                ChatPanel {
                    conversation_id: selected_conversation,
                    user_token: user_token,
                }
            }
            button {
                class: "fixed bottom-6 right-4 bg-black bg-opacity-50 z-10 md:hidden",
                onclick: move |_| {
                    is_sidebar_visible.set(!is_sidebar_visible());
                },
                "☰"
            }

            if is_sidebar_visible() {
                div {
                    class: "fixed left-0 top-0 w-3/4 dark:bg-gray-900 z-20 shadow-lg min-h-screen md:hidden",
                    ConversationsSidebar {
                        conversations: conversations.clone(),
                        selected_conversation: selected_conversation.clone(),
                        token: user_token(),
                        book_id: book_id_clone.clone(),
                    }
                }
            }
        }
    }
}
