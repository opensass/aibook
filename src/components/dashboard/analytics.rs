use crate::server::auth::controller::about_me;
use crate::server::book::controller::fetch_analytics_data;
use crate::server::book::response::AnalyticsData;
use dioxus::prelude::*;
use gloo_storage::SessionStorage;
use gloo_storage::Storage;

#[component]
pub fn AnalyticsPage() -> Element {
    let mut analytics = use_signal(|| AnalyticsData::default());
    let mut user_token = use_signal(|| "".to_string());
    let navigator = use_navigator();

    use_effect(move || {
        spawn(async move {
            let token: String = SessionStorage::get("jwt").unwrap_or_default();
            if token.is_empty() {
                navigator.push("/login");
            } else {
                match about_me(token.clone()).await {
                    Ok(data) => {
                        let _user = data.data.user;
                        user_token.set(token.clone());
                    }
                    Err(_) => {
                        navigator.push("/login");
                    }
                }
            }
        });
    });
    let _ = use_resource(move || async move {
        match fetch_analytics_data(user_token()).await {
            Ok(response) => {
                analytics.set(response.data);
            }
            Err(errr) => {
                dioxus_logger::tracing::error!("{}", errr.to_string());
            }
        }
    });

    rsx! {
        div {
            class: "pb-6",
            h1 { class: "text-3xl font-bold mb-6 text-gray-800 dark:text-gray-100", "Analytics" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6",
                MetricCard { title: "Total Books", value: analytics().engagement.total_books.to_string(), index: 0 }
                MetricCard { title: "Total Chapters", value: analytics().engagement.total_chapters.to_string(), index: 1 }
                MetricCard { title: "Avg Chapters per Book", value: format!("{:.2}", analytics().engagement.avg_chapters_per_book), index: 2 }
                MetricCard { title: "Trending Topic", value: analytics().predictions.trending_genre.clone(), index: 3 }
                MetricCard { title: "Projected Growth", value: format!("{:.2}%", analytics().predictions.projected_growth), index: 4 }
                MetricCard { title: "Avg Gen Time", value: format!("{:.2}s", analytics().ai_usage.avg_gen_time), index: 5 }
                MetricCard { title: "Success Rate", value: format!("{:.2}%", analytics().ai_usage.success_rate), index: 6 }
            }
        }
    }
}

#[component]
fn MetricCard(title: String, value: String, index: usize) -> Element {
    let card_shades = vec![
        "bg-gray-100 dark:bg-gray-900",
        "bg-gray-200 dark:bg-gray-800",
        "bg-gray-300 dark:bg-gray-700",
    ];

    let color = card_shades[index % card_shades.len()];

    rsx! {
        div {
            class: format!("p-6 rounded-lg shadow-lg transform transition hover:scale-105 hover:shadow-xl {}", color),
            h2 {
                class: "text-lg font-medium mb-2 text-gray-800 dark:text-gray-100",
                "{title}"
            }
            p {
                class: "text-2xl font-bold text-gray-900 dark:text-gray-50",
                "{value}"
            }
        }
    }
}
