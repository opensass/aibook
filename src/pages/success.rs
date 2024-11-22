use dioxus::prelude::*;

#[component]
pub fn SuccessPage() -> Element {
    rsx! {
        section {
            class: "min-h-screen flex flex-col justify-center items-center bg-gradient-to-br from-blue-500 via-indigo-600 to-purple-700 text-white",

            div {
                class: "text-center space-y-8 justify-center    ",

                div {
                    class: "flex justify-center items-center mx-auto w-24 h-24 bg-white rounded-full text-green-500 shadow-lg animate-bounce",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        class: "h-12 w-12",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M5 13l4 4L19 7",
                        }
                    }
                },

                h1 {
                    class: "text-4xl font-bold tracking-tight",
                    "Payment Successful!"
                },

                p {
                    class: "text-lg font-light text-gray-200 max-w-md mx-auto",
                    "Thank you for your payment. Your subscription is now active, and you can enjoy all the premium features we offer. A confirmation email has been sent to your registered email address."
                },

                a {
                    href: "/dashboard",
                    class: "inline-block px-8 py-3 bg-green-500 text-white font-semibold text-lg rounded-md shadow-md hover:bg-green-600 transition duration-300 ease-in-out",
                    "Go to Dashboard"
                }
            },

            footer {
                class: "mt-10 text-gray-300 text-sm",
                "© 2024 AIBook. All rights reserved."
            }
        }
    }
}
