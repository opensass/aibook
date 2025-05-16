use crate::components::common::header::Header;
use crate::components::toast::manager::{ToastManager, ToastType};
use crate::server::subscription::controller::start_stripe_payment;
use crate::server::subscription::request::StripePaymentRequest;
use chrono::Duration;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use gloo_storage::{SessionStorage, Storage};
use i18nrs::dioxus::I18nContext;
use serde_json::from_str;
use serde_json::Value;

#[derive(Props, Clone, PartialEq)]
struct PricingOption {
    title: String,
    price: &'static str,
    description: String,
    features: Vec<String>,
    highlight: bool,
    plan_id: Option<&'static str>,
}

#[component]
pub fn Pricing() -> Element {
    let I18nContext { i18n, .. } = use_context::<I18nContext>();
    let navigator = use_navigator();
    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let get_features = |key: &str| -> Vec<String> {
        let raw = i18n().t(key);
        from_str::<Vec<String>>(&raw).unwrap_or_else(|_| {
            tracing::warn!("Invalid array format for key '{}'", key);
            vec![]
        })
    };

    let pricing_options = vec![
        PricingOption {
            title: i18n().t("pricing.free.title"),
            price: "$0",
            description: i18n().t("pricing.free.description"),
            features: get_features("pricing.free.features"),
            highlight: false,
            plan_id: None,
        },
        PricingOption {
            title: i18n().t("pricing.monthly.title"),
            price: "$2/month",
            description: i18n().t("pricing.monthly.description"),
            features: get_features("pricing.monthly.features"),
            highlight: true,
            plan_id: Some("price_1QO1"),
        },
        PricingOption {
            title: i18n().t("pricing.yearly.title"),
            price: "$100/year",
            description: i18n().t("pricing.yearly.description"),
            features: get_features("pricing.yearly.features"),
            highlight: false,
            plan_id: Some("price_1QO1"),
        },
    ];

    let handle_plan_selection = move |plan: (Option<&'static str>, String)| {
        if let Some(plan_id) = plan.0 {
            spawn({
                let plan_title = plan.1;
                async move {
                    match start_stripe_payment(StripePaymentRequest {
                        plan_id: plan_id.to_string(),
                    })
                    .await
                    {
                        Ok(response) => {
                            SessionStorage::set("stripe", response.data.clone()).unwrap();
                            SessionStorage::set("method", "stripe").unwrap();
                            SessionStorage::set("plan", &plan_title).unwrap();
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Info".into(),
                                        i18n().t("pricing.toast.success").into(),
                                        ToastType::Info,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                            navigator.push(response.data);
                        }
                        Err(err) => tracing::error!("Stripe payment initiation failed: {:?}", err),
                    }
                }
            });
        } else {
            navigator.push("/login");
        }
    };

    rsx! {
        section {
            id: "pricing",
            class: "py-20 px-8 md:px-4 font-roboto flex min-h-screen justify-center dark:bg-gray-900 dark:text-white bg-white text-gray-900",

            div { class: "max-w-[1200px] mx-auto text-center",
                img {
                    src: asset!("/assets/pricing.webp"),
                    alt: "AIBook Pricing",
                    class: "w-32 h-32 mx-auto animate-bounce transition-transform duration-300 ease-in-out hover:scale-110 hover:rotate-12"
                }
                Header {
                    title: i18n().t("pricing.header.title"),
                    subtitle: i18n().t("pricing.header.subtitle")
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",

                    for option in pricing_options {
                        div { class: format!("p-6 rounded-lg border {}",
                            if option.highlight { "border-blue-500 bg-blue-50 relative shadow-lg text-black" } else { "border-gray-200" }),
                            if option.highlight {
                                div {
                                    class: "absolute top-0 right-0 bg-blue-500 text-white text-xs px-3 py-1 rounded-tr-md",
                                    "{i18n().t(\"pricing.best\")}"
                                }
                            }

                            h3 { class: "text-xl font-semibold", "{option.title}" },
                            p { class: "text-3xl font-bold mt-2 mb-4", "{option.price}" },
                            p { class: "mb-4 text-gray-600", "{option.description}" },

                            ul { class: "text-left space-y-2",
                                for feature in option.features.iter() {
                                    li { class: "flex items-center",
                                        span { class: "text-blue-500 mr-2", "✓" },
                                        "{feature}"
                                    }
                                }
                            },

                            button {
                                class: format!("mt-6 w-full py-2 rounded-md font-semibold {}",
                                    if option.highlight { "bg-blue-500 text-white hover:bg-blue-600" } else { "bg-gray-300 text-gray-700 hover:bg-gray-400" }),
                                onclick: move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                    handle_plan_selection((option.plan_id, option.title.clone()));
                                },
                                "{i18n().t(\"pricing.select\")}"
                            }
                        }
                    }
                }
            }
        }
    }
}
