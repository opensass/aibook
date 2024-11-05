use crate::components::features::Features;
use crate::components::footer::Footer;
use crate::components::hero::Hero;
use crate::components::testimonial::Testimonial;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "font-sans",
            Hero {}
            Features {}
            Testimonial {}
            Footer {}
        }
    }
}
