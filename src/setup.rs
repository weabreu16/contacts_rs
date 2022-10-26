use mongodb::{Client, Database};

pub async fn setup_database(uri: String) -> Database {

    let client = Client::with_uri_str(uri)
        .await
        .unwrap();

    let db = client.database("contacts-rs");

    db
}