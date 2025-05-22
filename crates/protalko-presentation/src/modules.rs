use std::sync::Arc;

use crate::config::Config;
use protalko_domain::repositories::Repositories;
use protalko_infrastructure::shared::{DefaultRepositories, DefaultRepositoriesError};

use protalko_use_case::user::UserUseCase;
use thiserror::Error;

pub struct Modules<R: Repositories> {
    config: Config,
    repositories: Arc<R>,

    user_use_case: UserUseCase<R>,
}

impl<R: Repositories> Modules<R> {
    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn repositories(&self) -> &Arc<R> {
        &self.repositories
    }

    pub fn user_use_case(&self) -> &UserUseCase<R> {
        &self.user_use_case
    }
}

#[derive(Debug, Error)]
pub enum DefaultModulesError {
    #[error(transparent)]
    DefaultRepositoriesError(#[from] DefaultRepositoriesError),
}
pub async fn default(config: Config) -> Result<Modules<DefaultRepositories>, DefaultModulesError> {
    let default_repositories = DefaultRepositories::new(config.postgres_url()).await?;
    let repositories = Arc::new(default_repositories);

    Ok(Modules {
        config,
        repositories: Arc::clone(&repositories),
        user_use_case: UserUseCase::new(Arc::clone(&repositories)),
    })
}
