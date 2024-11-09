#![allow(non_snake_case)]

use aibook::components::toast::provider::ToastProvider;
use aibook::router::Route;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        ToastProvider {
            Router::<Route> {}
        }
    }
}
