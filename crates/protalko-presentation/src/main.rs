mod config;
mod models;
mod modules;
mod routes;

use std::sync::Arc;

use axum::{routing::get, Router};
use config::Config;
use protalko_domain::{
    entities::user::UserId,
    repositories::{user::UserRepository, Repositories},
};
use uuid;

#[tokio::main]
async fn main() {
    async fn root_handler() -> String {
        "Hello, World!".to_string()
    }

    let config = Config::new().unwrap();
    let modules = modules::default(config).await.unwrap();
    let user = modules
        .repositories()
        .user_repository()
        .get_by_id(UserId::new(uuid::uuid!(
            "0a9d9cda-e3f8-49b9-8e55-b51c6c525216"
        )))
        .await;

    match user {
        Ok(u) => {
            println!("{:?}", u)
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }

    let app = Router::new()
        .route("/", get(root_handler))
        .with_state(Arc::new(modules));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
