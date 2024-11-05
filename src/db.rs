use mongodb::{options::ClientOptions, Client};
use std::env;
use tokio::sync::OnceCell;

static DB: OnceCell<Client> = OnceCell::const_new();

async fn init_db() -> Client {
    let conn = format!(
        "mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority",
        env::var("MONGODB_USR").expect("MONGODB_USR must be set."),
        env::var("MONGODB_PWD").expect("MONGODB_PWD must be set."),
        env::var("MONGODB_CLSTR").expect("MONGODB_CLSTR must be set."),
    );

    let mut client_options = ClientOptions::parse(conn)
        .await
        .expect("Client Options must be parsed.");
    client_options.app_name = Some(
        env::var("MONGODB_DB_NAME")
            .expect("MONGODB_DB_NAME must be set.")
            .to_string(),
    );
    let client = Client::with_options(client_options);

    client.expect("Client must be instantiated.")
}

pub async fn get_client() -> &'static Client {
    DB.get_or_init(init_db).await
}
