// use crate::components::common::server::JWT_TOKEN;
use crate::components::spinner::Spinner;
use crate::components::spinner::SpinnerSize;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::pages::login::{validate_email, validate_password};
use crate::router::Route;
use crate::server::auth::controller::{about_me, register_user};
use crate::server::auth::response::RegisterUserSchema;
use chrono::Duration;
use dioxus::prelude::*;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;
use input_rs::dioxus::Input;
use regex::Regex;

pub fn validate_name(field: String) -> bool {
    !&field.is_empty()
}

#[component]
pub fn Register() -> Element {
    let navigator = use_navigator();

    let mut name = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let mut error_message = use_signal(|| None::<String>);
    let mut email_valid = use_signal(|| true);
    let mut name_valid = use_signal(|| true);
    let mut password_valid = use_signal(|| true);
    let mut show_password = use_signal(|| false);

    let mut loading = use_signal(|| false);

    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if !token.is_empty() {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
                        navigator.push("/dashboard");
                    }
                    Err(e) => {
                        error_message.set(Some(e.to_string()));
                    }
                }
            }
        });
    });

    let handle_register = move |_| {
        loading.set(true);

        if !validate_email(email()) || password().is_empty() {
            error_message.set(Some(
                "Please provide a valid email and password.".to_string(),
            ));
            toasts_manager.set(
                toasts_manager()
                    .add_toast(
                        "Error".into(),
                        "Please provide a valid email and password!".into(),
                        ToastType::Error,
                        Some(Duration::seconds(5)),
                    )
                    .clone(),
            );
            loading.set(false);
            return;
        }

        spawn(async move {
            match register_user(RegisterUserSchema {
                name: name(),
                email: email(),
                password: password(),
            })
            .await
            {
                Ok(_) => {
                    toasts_manager.set(
                        toasts_manager()
                            .add_toast(
                                "Success".into(),
                                "Now, you can log in!".into(),
                                ToastType::Success,
                                Some(Duration::seconds(5)),
                            )
                            .clone(),
                    );
                    navigator.push("/login");
                    loading.set(false);
                }
                Err(e) => {
                    // error_message.set(Some(e.to_string()));
                    let msg = e.to_string();
                    let error_message = msg
                        .splitn(2, "error running server function:")
                        .nth(1)
                        .unwrap_or("")
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
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            class: "min-h-screen flex dark:bg-gray-900 dark:text-white bg-white text-gray-900",
            div {
                class: "md:flex-1 flex items-center justify-center bg-gradient-to-br from-blue-500 to-purple-600",
                style: "background-image: url('/assets/signup.webp'); background-size: cover; background-position: center;",
            }
            div {
                class: "flex-1 flex items-center justify-center p-8",
                form {
                    class: "w-full max-w-md",
                    onsubmit: handle_register,
                    Link {
                        to : Route::Home {},
                        class: "text-gray-400 text-sm",
                        "← Back to Home"
                    }
                    h1 { class: "text-3xl font-semibold mb-6 mt-4", "Register" },
                    div { class: "flex space-x-4 mb-6",
                        div { class: "flex flex-col items-start w-full",
                            span { class: "text-xs text-gray-500 mb-1", "Coming Soon" },
                            button {
                                class: "flex items-center justify-center w-full py-2 border rounded-md border-gray-300 bg-gray-100 text-gray-400 cursor-not-allowed",
                                disabled: "true",
                                "Login with Google"
                            }
                        }
                        div { class: "flex flex-col items-start w-full",
                            span { class: "text-xs text-gray-500 mb-1", "Coming Soon" },
                            button {
                                class: "flex items-center justify-center w-full py-2 border rounded-md border-gray-300 bg-gray-100 text-gray-400 cursor-not-allowed",
                                disabled: "true",
                                "Login with Github"
                            }
                        }
                    }
                    div { class: "text-center text-gray-500 mb-6", "or" }
                    // if let Some(error) = &error_message() {
                    //     p { class: "text-red-600 mb-4", "{error}" }
                    // }
                    Input {
                        r#type: "text",
                        label: "Full Name",
                        handle: name,
                        placeholder: "Enter your name",
                        error_message: "Full name can't be blank!",
                        required: true,
                        valid_handle: name_valid,
                        validate_function: validate_name,
                        class: "mt-1 block w-full shadow-sm dark:bg-gray-900",
                        field_class: "relative validate-input mb-6",
                        label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                        input_class: {
                            if name_valid() {
                                "dark:border-gray-300 dark:bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                            } else {
                                "border-red-500 bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                            }
                        },
                        error_class: "text-red-500 text-sm",
                    }
                    Input {
                        r#type: "text",
                        label: "Email Address",
                        handle: email,
                        placeholder: "Email Address",
                        error_message: "Email address can't be blank!",
                        required: true,
                        valid_handle: email_valid,
                        validate_function: validate_email,
                        class: "mt-1 block w-full shadow-sm dark:bg-gray-900",
                        field_class: "relative validate-input mb-6",
                        label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                        input_class: {
                            if email_valid() {
                                "dark:border-gray-300 dark:bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                            } else {
                                "border-red-500 bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                            }
                        },
                        error_class: "text-red-500 text-sm",
                    }
                    Input {
                        r#type: "password",
                        label: "Password",
                        handle: password,
                        placeholder: "Password",
                        error_message: "Password can't be blank!",
                        required: true,
                        valid_handle: password_valid,
                        validate_function: validate_password,
                        class: "mt-1 block w-full shadow-sm dark:bg-gray-900",
                        field_class: "relative validate-input mb-6",
                        label_class: "block text-sm font-medium dark:text-gray-300 text-gray-700",
                        input_class: {
                            if password_valid() {
                                "dark:border-gray-300 dark:bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                            } else {
                                "border-red-500 bg-gray-900 h-12 block w-full px-4 py-2 border rounded-md shadow-sm"
                            }
                        },
                        eye_active: "cursor-pointer absolute right-4 top-3 text-xl text-gray-600 toggle-button fa fa-eye",
                        eye_disabled: "cursor-pointer absolute right-4 top-3 text-xl text-gray-600 toggle-button fa fa-eye-slash",
                        error_class: "text-red-500 text-sm",
                    }
                    button {
                        class: "flex items-center text-center justify-center space-x-2 w-full py-2 mt-4 bg-blue-600 hover:bg-blue-700 text-white rounded-md",
                        r#type: "submit",
                        disabled: loading(),
                        if loading() {
                            Spinner {
                                aria_label: "Loading spinner".to_string(),
                                size: SpinnerSize::Md,
                                dark_mode: true,
                            }
                            span { "Signing Up..." }
                        } else {
                            span { "Sign Up" }
                        }
                    }
                }
            }
        }
    }
}
