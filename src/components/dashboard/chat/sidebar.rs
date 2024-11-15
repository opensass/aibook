use crate::server::conversation::controller::create_conversation;
use crate::server::conversation::controller::get_conversations;
use crate::server::conversation::model::Conversation;
use crate::server::conversation::request::CreateConversationRequest;
use crate::server::conversation::request::GetConversationsRequest;
use crate::theme::Theme;
use bson::oid::ObjectId;
use chrono::Utc;

use dioxus::prelude::*;

#[component]
pub fn ConversationsSidebar(
    conversations: Signal<Vec<Conversation>>,
    selected_conversation: Signal<ObjectId>,
    token: String,
    book_id: String,
) -> Element {
    let token_clone = token.clone();
    let book_id_clone = book_id.clone();
    let theme = use_context::<Signal<Theme>>();

    use_effect(move || {
        let token = token.clone();
        let book_id = book_id.clone();

        spawn(async move {
            if let Ok(conv_list) =
                get_conversations(GetConversationsRequest { token, book_id }).await
            {
                conversations.set(conv_list.data);
            }
        });
    });

    rsx! {
        div {
            class: format!("p-4 {}", if theme() == Theme::Dark { "border-gray-600 bg-gray-900" } else { "border-gray-200" }),

            h3 { class: "text-lg font-semibold mb-4 text-blue-500", "Conversations" }

            button {
                class: "w-full bg-blue-500 text-white p-2 rounded-lg mb-4",
                onclick: move |_| {
                    let token_clone = token_clone.clone();
                    let book_id_clone = book_id_clone.clone();
                    spawn(async move {
                        let title = format!("Conversation {}", Utc::now().timestamp());
                        if let Ok(new_conversation) = create_conversation(CreateConversationRequest { book_id: book_id_clone, token: token_clone, title }).await {
                            let mut current_conversations = conversations();
                            current_conversations.push(new_conversation.data);
                            conversations.set(current_conversations);
                        }
                    });
                },
                "+ New Chat"
            }

            ul {
                for conversation in conversations().into_iter() {
                    li {
                        class: format!("p-2 cursor-pointer rounded-lg {}",
                            if conversation.id == selected_conversation() {
                                "bg-blue-200 text-black dark:bg-blue-600 font-bold"
                            } else {
                                "hover:bg-blue-100 hover:text-black dark:hover:bg-blue-800"
                            }
                        ),
                        onclick: move |_| selected_conversation.set(conversation.id),
                        "{conversation.title}"
                    }
                }
            }
        }
    }
}
