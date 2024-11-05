use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct StarRatingProps {
    star_images: Vec<&'static str>,
}

#[component]
pub fn StarRating(props: StarRatingProps) -> Element {
    rsx! {
        div { class: "flex justify-center mb-4",
            for star in props.star_images {
                img { src: "{star}", class: "w-5 h-5" }
            }
        }
    }
}
