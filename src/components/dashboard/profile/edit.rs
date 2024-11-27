use crate::components::dashboard::fields::input::InputField;
use crate::components::dashboard::profile::view::ProfileDetailsProps;
use crate::components::toast::manager::{ToastManager, ToastType};
use crate::server::auth::controller::edit_profile;
use crate::server::auth::request::EditUserSchema;
use chrono::Duration;
use dioxus::prelude::*;

#[component]
pub fn ProfileForm(props: ProfileDetailsProps) -> Element {
    let user = &props.user;
    let dark_mode = props.dark_mode;
    let user_token = props.user_token;

    let name = use_signal(|| user.name.clone());
    // default to lord Ferris
    let photo =
        use_signal(|| "https://rustacean.net/assets/rustacean-orig-noshadow.svg".to_string());
    let email = use_signal(|| user.email.clone());
    let old_password = use_signal(|| String::new());
    let new_password = use_signal(|| String::new());
    let confirm_password = use_signal(|| String::new());

    let mut name_valid = use_signal(|| true);
    let validate_name = |name: &str| !name.is_empty();
    let mut email_valid = use_signal(|| true);
    let validate_email = |email: &str| email.contains("@") && email.contains(".");

    let photo_valid = use_signal(|| true);
    let validate_photo = |photo: &str| !photo.is_empty();

    let mut old_password_valid = use_signal(|| true);
    let validate_old_password = |password: &str| !password.is_empty();
    let mut new_password_valid = use_signal(|| true);
    let validate_new_password = |password: &str| password.len() >= 8;
    let mut confirm_password_valid = use_signal(|| true);
    let validate_confirm_password = |confirm: &str, new: &str| confirm == new;

    let navigator = use_navigator();
    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let handle_submit = move |evt: Event<FormData>| {
        evt.stop_propagation();
        let user_token = user_token.clone();

        let mut all_valid = true;

        if !validate_name(&name()) {
            name_valid.set(false);
            all_valid = false;
        } else {
            name_valid.set(true);
        }

        if !validate_email(&email()) {
            email_valid.set(false);
            all_valid = false;
        } else {
            email_valid.set(true);
        }

        if !validate_old_password(&old_password()) {
            old_password_valid.set(false);
            all_valid = false;
        } else {
            old_password_valid.set(true);
        }

        if !validate_new_password(&new_password()) {
            new_password_valid.set(false);
            all_valid = false;
        } else {
            new_password_valid.set(true);
        }

        if !validate_confirm_password(&confirm_password(), &new_password()) {
            confirm_password_valid.set(false);
            all_valid = false;
        } else {
            confirm_password_valid.set(true);
        }

        if all_valid {
            spawn({
                async move {
                    match edit_profile(EditUserSchema {
                        token: user_token,
                        name: Some(name()),
                        email: Some(email()),
                        photo: Some(photo()),
                        old_password: Some(old_password()),
                        new_password: Some(new_password()),
                        confirm_password: Some(confirm_password()),
                    })
                    .await
                    {
                        Ok(_) => {
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Success".into(),
                                        "Profile updated successfully.".into(),
                                        ToastType::Success,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                            navigator.push("/dashboard");
                        }
                        Err(e) => {
                            let msg = e.to_string();
                            let error_message = msg
                                .splitn(2, "error running server function:")
                                .nth(1)
                                .unwrap_or("An error occurred")
                                .trim();
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Error".into(),
                                        error_message.into(),
                                        ToastType::Error,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                        }
                    }
                }
            });
        } else {
            toasts_manager.set(
                toasts_manager()
                    .add_toast(
                        "Error".into(),
                        "Please ensure all fields are valid.".into(),
                        ToastType::Error,
                        Some(Duration::seconds(5)),
                    )
                    .clone(),
            );
        }
    };

    rsx!(
        form { class: "space-y-4",
            onsubmit: handle_submit,
            InputField {
                label: "Name",
                value: name,
                is_valid: name_valid,
                validate: validate_name,
                required: true
            },
            InputField {
                label: "Image",
                value: photo,
                is_valid: photo_valid,
                validate: validate_photo,
                required: true
            },
            InputField {
                label: "Email",
                value: email,
                is_valid: email_valid,
                validate: validate_email,
                required: true
            },
            InputField {
                label: "Old Password",
                value: old_password,
                is_valid: old_password_valid,
                validate: validate_old_password,
                required: true
            },
            InputField {
                label: "New Password",
                value: new_password,
                is_valid: new_password_valid,
                validate: validate_new_password,
                required: true
            },
            InputField {
                label: "Confirm Password",
                value: confirm_password,
                is_valid: confirm_password_valid,
                validate: validate_new_password,
                required: true
            },
            button {
                class: format!("py-2 px-4 rounded-md {}", if dark_mode { "bg-blue-600" } else { "bg-blue-500 text-white" }),
                r#type: "submit",
                "Save"
            }
        }
    )
}
