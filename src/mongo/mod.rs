pub mod models;
pub mod controllers;

use mongodb::{Client};

pub const DB_URI: &str = "mongodb://rust:rust@127.0.0.1:27017/rust_test";
pub const DB_NAME: &str = "rust_test";

pub async fn mongo_connect() -> Client {
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| DB_URI.into());
    Client::with_uri_str(uri).await.expect("failed to create mongo client")
}