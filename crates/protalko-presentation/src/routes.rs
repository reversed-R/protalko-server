use std::sync::Arc;

use axum::{routing::get, Router};
use protalko_infrastructure::shared::DefaultRepositories;

use crate::modules::Modules;

pub mod user;

pub fn router(modules: Arc<Modules<DefaultRepositories>>) -> Router {
    Router::new()
        .route("/users/{user_id}", get(user::handle_get_by_id))
        .with_state(modules)
}
