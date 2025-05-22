pub mod health;
pub mod user;

use crate::modules::Modules;
use protalko_infrastructure::shared::DefaultRepositories;

use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(modules: Arc<Modules<DefaultRepositories>>) -> Router {
    Router::new()
        .route("/health", get(health::handle_health))
        .route("/users/{user_id}", get(user::handle_get_by_id))
        .with_state(modules)
}
