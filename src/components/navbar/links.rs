use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum NavLink {
    HomePage,
    Features,
    Pricing,
    Testimonials,
}

#[component]
pub fn NavLinks(show_items: bool) -> Element {
    let mut active_link = use_signal(|| NavLink::HomePage);

    let is_active = |link: &NavLink| {
        if active_link() == *link {
            "active-underline"
        } else {
            ""
        }
    };

    let nav_links = vec![
        (NavLink::HomePage, "#home", "Home"),
        (NavLink::Features, "#features", "Features"),
        (NavLink::Pricing, "#pricing", "Pricing"),
        (NavLink::Testimonials, "#testimonial", "Testimonial"),
    ];
    if show_items {
        return {
            rsx! {
                div {
                    class: "flex flex-col md:flex-row gap-4 md:gap-8 mr-8 md:mb-0 mb-8",
                    for (link, href, label) in nav_links {
                        a {
                            href: href,
                            class: format!(
                                "transition-colors duration-300 hover:text-gray-500  {}",

                            is_active(&link)
                            ),
                            onclick: move |_| active_link.set(link.clone()),
                            "{label}"
                        }
                    }
                }
            }
        };
    } else {
        return rsx! {};
    };
}
