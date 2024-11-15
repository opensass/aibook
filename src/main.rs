#![allow(non_snake_case)]

use aibook::components::toast::provider::ToastProvider;
use aibook::router::Route;
use aibook::theme::ThemeProvider;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");

    #[cfg(feature = "web")]
    {
        let config = dioxus_web::Config::new().hydrate(true);
        LaunchBuilder::new().with_cfg(config).launch(App);
    }

    #[cfg(feature = "server")]
    {
        use aibook::db::get_client;
        use axum::{Extension, Router};
        use std::sync::Arc;
        use tower_http::cors::CorsLayer;

        #[derive(Clone)]
        #[allow(dead_code)]
        pub struct AppState {
            client: mongodb::Client,
        }

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let client = get_client().await;

                let state = Arc::new(AppState {
                    client: client.clone(),
                });

                let app = Router::new()
                    .layer(CorsLayer::permissive())
                    .layer(Extension(state))
                    .serve_dioxus_application(ServeConfig::builder().build(), || {
                        VirtualDom::new(App)
                    })
                    .await;

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
        ThemeProvider {
            ToastProvider {
                Router::<Route> {}
            }
        }
    }
}
