pub mod manager;
pub mod provider;

use crate::components::toast::manager::Toast;
use crate::components::toast::manager::ToastType;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ToastProps {
    pub toast: Toast,
    // pub onclose: Event<MouseData>
}

#[component]
pub fn Toast(props: ToastProps) -> Element {
    let color = match props.toast.toast_type {
        ToastType::Info => "bg-blue-600",
        ToastType::Success => "bg-green-600",
        ToastType::Warning => "bg-yellow-600",
        ToastType::Error => "bg-red-600",
    };

    rsx! {
        div {
            class: "rounded-lg shadow-lg p-4 bg-opacity-90 flex flex-col justify-center items-start transition-opacity duration-300 ease-out text-white z-50 {color}",

            div {
                class: "text-xl font-semibold",
                "{props.toast.title}"
            }

            div {
                class: "text-base italic mt-2 leading-relaxed",
                "{props.toast.body}"
            }
            // div{
            //   class:"cursor-pointer mt-2",
            //   onclick: props.onclose,
            //   "X"
            // }
        }
    }
}
