use crate::components::testimonial::author::AuthorInfo;
use crate::components::testimonial::rating::StarRating;
use crate::components::testimonial::TestimonialData;
use dioxus::prelude::*;

#[component]
pub fn TestimonialCard(testimonial: TestimonialData) -> Element {
    rsx! {
        div { class: "bg-white p-8 rounded-lg shadow-lg max-w-lg text-center border border-black",
            StarRating { star_images: testimonial.star_images.clone() }
            blockquote { class: "text-xl font-semibold text-black mb-4", "{testimonial.quote}" }
            AuthorInfo {
                author_image: testimonial.author_image,
                author_name: testimonial.author_name,
                author_title: testimonial.author_title,
                company_logo: testimonial.company_logo,
            }
        }
    }
}
