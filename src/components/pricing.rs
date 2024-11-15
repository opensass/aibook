use crate::components::common::header::Header;
use crate::theme::Theme;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
struct PricingOption {
    title: &'static str,
    price: &'static str,
    description: &'static str,
    features: Vec<&'static str>,
    highlight: bool,
}

#[component]
pub fn Pricing() -> Element {
    let dark_mode = use_context::<Signal<Theme>>();
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
        },
    ];

    rsx! {
        section {
            id: "pricing",
            class: format!("py-20 px-8 md:px-4 font-roboto flex min-h-screen justify-center {}",
                if dark_mode() == Theme::Dark { "bg-gray-900 text-white" } else { "bg-white text-gray-900" }),

            div { class: "max-w-[1200px] mx-auto text-center",
                img {
                    src: "./pricing.webp",
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

                            button { class: format!("mt-6 w-full py-2 rounded-md font-semibold {}",
                                if option.highlight { "bg-blue-500 ttext-gray-700 hover:bg-blue-600" } else { "bg-gray-300 text-gray-700 hover:bg-gray-400" }),
                                "Select Plan"
                            }
                        }
                    }
                }
            }
        }
    }
}
