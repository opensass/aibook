use crate::server::auth::model::User;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProfileDetailsProps {
    pub user: User,
    pub dark_mode: bool,
    pub user_token: String,
}

#[component]
pub fn ProfileDetails(props: ProfileDetailsProps) -> Element {
    rsx!(
        div { class: "grid grid-cols-2 gap-4 md:grid-cols-3",
            div { class: "flex items-center space-x-2",
                span { class: "font-bold", "User ID:" }
                span { "{props.user.id}" }
            }
            div { class: "flex items-center space-x-2",
                span { class: "font-bold", "Name:" }
                span { "{props.user.name}" }
            }
            div { class: "flex items-center space-x-2",
                span { class: "font-bold", "Email:" }
                span { "{props.user.email}" }
            }
            div { class: "flex items-center space-x-2",
                span { class: "font-bold", "Role:" }
                span { "{props.user.role}" }
            }
            div { class: "flex items-center space-x-2",
                span { class: "font-bold", "Verified:" }
                span { "{props.user.verified}" }
            }
            div { class: "flex items-center space-x-2",
                span { class: "font-bold", "Registered At:" }
                span { "{props.user.created_at.format(\"%B %d, %Y\")}" }
            }
        }
    )
}
