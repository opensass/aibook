#![allow(non_snake_case)]

use aibook::components::toast::provider::ToastProvider;
use aibook::router::Route;
use aibook::theme::ThemeProvider;
use dioxus::prelude::*;
use dioxus_logger::tracing;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    #[cfg(feature = "web")]
    {
        dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
        tracing::info!("starting app");
        let config = dioxus_web::Config::new().hydrate(true);

        LaunchBuilder::new().with_cfg(config).launch(App);
    }

    #[cfg(feature = "server")]
    {
        use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
        use axum::http::Method;
        use axum::{Extension, Router};
        use dotenv::dotenv;
        use std::sync::Arc;
        use tower_http::cors::{Any, CorsLayer};

        dotenv().ok();
        dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let cors = CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

                let app = Router::new()
                    .layer(cors)
                    .serve_dioxus_application(ServeConfig::new().unwrap(), App);

                let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }
}

fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: "https://kit.fontawesome.com/62e08d355c.js" }
        document::Link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        ThemeProvider {
            ToastProvider {
                Router::<Route> {}
            }
        }
    }
}
