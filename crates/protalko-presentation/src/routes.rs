pub mod health;
pub mod user;

use crate::modules::Modules;
use protalko_infrastructure::shared::DefaultRepositories;

use axum::{routing::get, Router};
use std::sync::Arc;
use utoipa::OpenApi;

pub fn router(modules: Arc<Modules<DefaultRepositories>>) -> Router {
    Router::new()
        .route("/health", get(health::handle_health))
        .route("/users/{user_id}", get(user::handle_get_by_id))
        .with_state(modules)
}

use crate::routes;
#[derive(OpenApi)]
#[openapi(
    info(title = "Protalko API"),
    tags(
        (name = "health", description = "APIの正常性チェック"),
        (name = "user", description = "ユーザー関連の操作"),
    ), 
    paths(
        routes::health::handle_health,
        routes::user::handle_get_by_id
    )
)]
pub struct ApiDocs;
