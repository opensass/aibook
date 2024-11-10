#![allow(dead_code)]

use dioxus::prelude::*;

#[derive(Clone, Default, Debug, PartialEq)]
pub enum SpinnerSize {
    #[default]
    None,
    Sm,
    Md,
    Lg,
    Xl,
    Custom(String),
}

impl SpinnerSize {
    fn as_class(&self) -> &'static str {
        match self {
            SpinnerSize::None => "",
            SpinnerSize::Sm => "w-4 h-4",
            SpinnerSize::Md => "w-6 h-6",
            SpinnerSize::Lg => "w-8 h-8",
            SpinnerSize::Xl => "w-12 h-12",
            SpinnerSize::Custom(_) => "",
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SpinnerProps {
    #[props(default = "Loading...".to_string())]
    aria_label: String,
    #[props(default)]
    size: SpinnerSize,
    #[props(default = false)]
    dark_mode: bool,
}

#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    let size_classes = props.size.as_class();

    let style = if let SpinnerSize::Custom(diameter) = &props.size {
        format!("width: {}; height: {};", diameter, diameter)
    } else {
        String::new()
    };

    let color_class = if props.dark_mode {
        "text-white border-t-white"
    } else {
        "text-blue-500 border-t-blue-500"
    };

    rsx! {
        svg {
            class: "animate-spin {size_classes} {color_class}",
            style: "{style}",
            role: "progressbar",
            view_box: "0 0 50 50",
            "aria-label": "{props.aria_label}",
            circle {
                class: "stroke-current text-gray-300 dark:text-gray-600",
                cx: "25",
                cy: "25",
                r: "20",
                fill: "none",
                stroke_width: "4"
            },
            circle {
                class: "{color_class}",
                cx: "25",
                cy: "5",
                r: "3",
            }
        }
    }
}
