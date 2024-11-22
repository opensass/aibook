use std::env;
use stripe::Client;
use tokio::sync::{Mutex, OnceCell};

static PAY: OnceCell<Mutex<Client>> = OnceCell::const_new();

async fn init_stripe() -> &'static Mutex<Client> {
    PAY.get_or_init(|| async {
        let client =
            Client::new(env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set."));
        Mutex::new(client)
    })
    .await
}

pub async fn get_stripe() -> &'static Mutex<Client> {
    init_stripe().await
}
