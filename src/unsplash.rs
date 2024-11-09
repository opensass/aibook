use http_api_isahc_client::IsahcClient;
use tokio::sync::{Mutex, OnceCell};

static UNSPLASH: OnceCell<Mutex<IsahcClient>> = OnceCell::const_new();

async fn init_unsplash() -> &'static Mutex<IsahcClient> {
    UNSPLASH
        .get_or_init(|| async {
            let client = IsahcClient::new().unwrap();
            Mutex::new(client)
        })
        .await
}

pub async fn get_unsplash_client() -> &'static Mutex<IsahcClient> {
    init_unsplash().await
}
