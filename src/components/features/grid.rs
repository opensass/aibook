use crate::components::features::item::FeatureItem;
use crate::components::features::Feature;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FeatureGridProps {
    features: Vec<Feature>,
}

#[component]
pub fn Grid(props: FeatureGridProps) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8 mt-10",
            for feature in &props.features {
                FeatureItem {
                    icon: feature.icon.clone(),
                    title: feature.title,
                    description: feature.description,
                }
            }
        }
    }
}
