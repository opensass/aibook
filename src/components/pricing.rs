use crate::components::common::header::Header;
use crate::components::toast::manager::ToastManager;
use crate::components::toast::manager::ToastType;
use crate::server::subscription::controller::start_stripe_payment;
use crate::server::subscription::request::StripePaymentRequest;
use chrono::Duration;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;

#[derive(Props, Clone, PartialEq)]
struct PricingOption {
    title: &'static str,
    price: &'static str,
    description: &'static str,
    features: Vec<&'static str>,
    highlight: bool,
    plan_id: Option<&'static str>,
}

#[component]
pub fn Pricing() -> Element {
    let navigator = use_navigator();
    let mut toasts_manager = use_context::<Signal<ToastManager>>();

    let pricing_options = vec![
        PricingOption {
            title: "Free",
            price: "$0",
            description: "Sign up for free",
            features: vec![
                "Access to 10 prompts per month",
                "Basic content generation tools",
                "Google Gemini Pro (limited features)",
            ],
            highlight: false,
            plan_id: None,
        },
        PricingOption {
            title: "Monthly",
            price: "$2/month",
            description: "For frequent creators",
            features: vec![
                "1000 prompts per month",
                "Full access to Google Gemini Pro",
                "Vision AI for image descriptions",
                "Priority customer support",
            ],
            highlight: true,
            // TODO: Change to env var
            plan_id: Some("price_1QO1"),
        },
        PricingOption {
            title: "Yearly",
            price: "$100/year",
            description: "Unlimited access",
            features: vec![
                "Unlimited prompts",
                "All content creation tools",
                "Google Gemini 1.5 Pro",
                "Vision AI for image descriptions",
                "Advanced analytics dashboard",
                "Priority support",
            ],
            highlight: false,
            // TODO: Change to env var
            plan_id: Some("price_1QO1"),
        },
    ];
    let handle_plan_selection = move |plan: (Option<&'static str>, &'static str)| {
        if let Some(plan_id) = plan.0 {
            spawn({
                let plan_title = plan.1.to_string();
                async move {
                    match start_stripe_payment(StripePaymentRequest {
                        plan_id: plan_id.to_string(),
                    })
                    .await
                    {
                        Ok(response) => {
                            SessionStorage::set("stripe", response.data.clone())
                                .expect("Session storage failed");
                            SessionStorage::set("method", "stripe")
                                .expect("Session storage failed");
                            SessionStorage::set("plan", &plan_title)
                                .expect("Session storage failed");
                            toasts_manager.set(
                                toasts_manager()
                                    .add_toast(
                                        "Info".into(),
                                        "Stripe payment initiation success!".into(),
                                        ToastType::Info,
                                        Some(Duration::seconds(5)),
                                    )
                                    .clone(),
                            );
                            navigator.push(response.data);
                        }
                        Err(err) => {
                            tracing::error!("Stripe payment initiation failed: {:?}", err);
                        }
                    }
                }
            });
        } else {
            navigator.push("/login");
            tracing::info!("Free plan selected.");
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
                    title: "Get full access to AIBook",
                    subtitle: "Choose the plan that suits your content creation needs."
                }
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",

                    for option in pricing_options {
                        div { class: format!("p-6 rounded-lg border {}",
                            if option.highlight { "border-blue-500 bg-blue-50 relative shadow-lg text-black" } else { "border-gray-200" }),
                            if option.highlight {
                                div {
                                    class: "absolute top-0 right-0 bg-blue-500 text-white text-xs px-3 py-1 rounded-tr-md",
                                    "Best Package"
                                }
                            }

                            h3 { class: "text-xl font-semibold", "{option.title}" },
                            p { class: "text-3xl font-bold mt-2 mb-4", "{option.price}" },
                            p { class: "mb-4 text-gray-600", "{option.description}" },

                            ul { class: "text-left space-y-2",
                                for feature in option.features {
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
                                    handle_plan_selection((option.plan_id, option.title));
                                },
                                "Select Plan"
                            }
                        }
                    }
                }
            }
        }
    }
}
