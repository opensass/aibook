use crate::server::book::controller::fetch_analytics_data;
use crate::server::book::response::AnalyticsData;
use dioxus::prelude::*;

#[component]
pub fn AnalyticsPage() -> Element {
    let mut analytics = use_signal(|| AnalyticsData::default());
    let _ = use_resource(move || async move {
        match fetch_analytics_data().await {
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
            h1 { class: "text-3xl font-bold mb-6", "Analytics" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6",
                MetricCard { title: "Total Books", value: analytics().engagement.total_books.to_string() }
                MetricCard { title: "Total Chapters", value: analytics().engagement.total_chapters.to_string() }
                MetricCard { title: "Avg Chapters per Book", value: format!("{:.2}", analytics().engagement.avg_chapters_per_book) }
                MetricCard { title: "Trending Topic", value: analytics().predictions.trending_genre.clone() }
                MetricCard { title: "Projected Growth", value: format!("{:.2}%", analytics().predictions.projected_growth) }
                MetricCard { title: "Avg Gen Time", value: format!("{:.2}s", analytics().ai_usage.avg_gen_time) }
                MetricCard { title: "Success Rate", value: format!("{:.2}%", analytics().ai_usage.success_rate) }
            }
        }
    }
}

#[component]
fn MetricCard(title: String, value: String) -> Element {
    rsx! {
        div {
            class: "p-4 rounded-lg shadow-md",
            h2 { class: "text-lg font-medium mb-2", "{title}" }
            p { class: "text-2xl font-bold", "{value}" }
        }
    }
}
