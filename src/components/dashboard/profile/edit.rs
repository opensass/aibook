use crate::components::dashboard::profile::view::ProfileDetailsProps;
use crate::components::toast::manager::{ToastManager, ToastType};
use crate::server::auth::controller::edit_profile;
use crate::server::auth::request::EditUserSchema;
use chrono::Duration;
use dioxus::prelude::*;
use input_rs::dioxus::Input;

fn validate_name(name: String) -> bool {
    !name.is_empty()
}

fn validate_email(email: String) -> bool {
    email.contains('@') && email.contains('.')
}

fn validate_photo(photo: String) -> bool {
    !photo.is_empty()
}

fn validate_old_password(password: String) -> bool {
    !password.is_empty()
}

fn validate_new_password(password: String) -> bool {
    password.len() >= 8
}

fn validate_confirm_password(confirm: String, new: String) -> bool {
    confirm == new
}

#[component]
pub fn ProfileForm(props: ProfileDetailsProps) -> Element {
    let user = &props.user;
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
    let mut email_valid = use_signal(|| true);

    let photo_valid = use_signal(|| true);

    let mut old_password_valid = use_signal(|| true);
    let mut new_password_valid = use_signal(|| true);
    let mut confirm_password_valid = use_signal(|| true);

    let navigator = use_navigator();
    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let handle_submit = move |evt: Event<FormData>| {
        evt.stop_propagation();
        let user_token = user_token.clone();

        let mut all_valid = true;

        if !validate_name(name()) {
            name_valid.set(false);
            all_valid = false;
        } else {
            name_valid.set(true);
        }

        if !validate_email(email()) {
            email_valid.set(false);
            all_valid = false;
        } else {
            email_valid.set(true);
        }

        if !validate_old_password(old_password()) {
            old_password_valid.set(false);
            all_valid = false;
        } else {
            old_password_valid.set(true);
        }

        if !validate_new_password(new_password()) {
            new_password_valid.set(false);
            all_valid = false;
        } else {
            new_password_valid.set(true);
        }

        if !validate_confirm_password(confirm_password(), new_password()) {
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
            Input {
                r#type: "text",
                label: "Name",
                handle: name,
                placeholder: "Name",
                error_message: "Name can't be blank!",
                required: true,
                valid_handle: name_valid,
                validate_function: validate_name,
                class: "field mb-6",
                field_class: "validate-input mb-6",
                label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                input_class: {if name_valid() {
                    "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                } else {
                    "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                }},
                error_class: "text-red-500 text-sm mt-1",
            }
            ,
            Input {
                r#type: "text",
                label: "Image",
                handle: photo,
                placeholder: "Image URL",
                error_message: "Image can't be blank!",
                required: true,
                valid_handle: photo_valid,
                validate_function: validate_photo,
                class: "field mb-6",
                field_class: "validate-input mb-6",
                label_class: "block text-sm font-medium dark:text-gray-300 block text-sm font-medium text-gray-700",
                input_class: {if photo_valid() {
                    "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                } else {
                    "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                }},
                error_class: "text-red-500 text-sm mt-1",
            }
            ,
            Input {
                r#type: "email",
                label: "Email",
                handle: email,
                placeholder: "Email",
                error_message: "Enter a valid email address!",
                required: true,
                valid_handle: email_valid,
                validate_function: validate_email,
                class: "field mb-6",
                field_class: "validate-input mb-6",
                label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                input_class: {if email_valid() {
                    "dark:border-gray-300 dark:bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                } else {
                    "border-red-500 bg-gray-900 mt-1 block w-full p-2 border rounded-md shadow-sm"
                }},
                error_class: "text-red-500 text-sm mt-1",
            }
            ,
            Input {
                r#type: "password",
                label: "Old Password",
                handle: old_password,
                placeholder: "Old Password",
                error_message: "Old password can't be blank!",
                required: true,
                valid_handle: old_password_valid,
                validate_function: validate_old_password,
                class: "field mb-6",
                field_class: "relative validate-input mb-6",
                label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                input_class: {
                    if old_password_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                    }
                },
                eye_active: "cursor-pointer absolute right-4 top-3 text-xl text-gray-600 toggle-button fa fa-eye",
                eye_disabled: "cursor-pointer absolute right-4 top-3 text-xl text-gray-600 toggle-button fa fa-eye-slash",
                error_class: "text-red-500 text-sm mt-1",
            }
            ,
            Input {
                r#type: "password",
                label: "New Password",
                handle: new_password,
                placeholder: "New Password",
                error_message: "Password must be at least 8 characters!",
                required: true,
                valid_handle: new_password_valid,
                validate_function: validate_new_password,
                class: "field mb-6",
                field_class: "relative validate-input mb-6",
                label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                input_class: {
                    if new_password_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                    }
                },
                eye_active: "cursor-pointer absolute right-4 top-3 text-xl text-gray-600 toggle-button fa fa-eye",
                eye_disabled: "cursor-pointer absolute right-4 top-3 text-xl text-gray-600 toggle-button fa fa-eye-slash",
                error_class: "text-red-500 text-sm mt-1",
            },
            Input {
                r#type: "password",
                label: "Confirm Password",
                handle: confirm_password,
                placeholder: "Confirm Password",
                error_message: "Passwords do not match!",
                required: true,
                valid_handle: confirm_password_valid,
                validate_function: validate_new_password,
                class: "field mb-6",
                field_class: "relative validate-input mb-6",
                label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                input_class: {
                    if confirm_password_valid() {
                        "dark:border-gray-300 dark:bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                    } else {
                        "border-red-500 bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                    }
                },
                eye_active: "cursor-pointer absolute right-4 top-3 text-xl text-gray-600 toggle-button fa fa-eye",
                eye_disabled: "cursor-pointer absolute right-4 top-3 text-xl text-gray-600 toggle-button fa fa-eye-slash",
                error_class: "text-red-500 text-sm mt-1",
            }
            button {
                class: "py-2 px-4 rounded-md dark:bg-blue-600 bg-blue-500 text-white",
                r#type: "submit",
                "Save"
            }
        }
    )
}
