use gems::Client;
use std::env;
use tokio::sync::{Mutex, OnceCell};

static AI: OnceCell<Mutex<Client>> = OnceCell::const_new();

async fn init_ai_with_model(model: String) -> &'static Mutex<Client> {
    AI.get_or_init(|| async {
        let client = Client::new(
            &env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set."),
            &model,
        );
        Mutex::new(client)
    })
    .await
}

pub async fn get_ai(model: String) -> &'static Mutex<Client> {
    init_ai_with_model(model).await
}
