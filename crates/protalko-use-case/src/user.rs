use std::sync::Arc;

use protalko_domain::repositories::Repositories;

pub mod operations;

pub struct UserUseCase<R: Repositories> {
    repositories: Arc<R>,
}

impl<R: Repositories> UserUseCase<R> {
    pub fn new(repositories: Arc<R>) -> Self {
        Self { repositories }
    }
}
