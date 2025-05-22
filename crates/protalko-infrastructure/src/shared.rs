pub mod postgresql;

use crate::{shared::postgresql::Postgresql, user::PgUserRepository};
use postgresql::PostgresqlError;
use protalko_domain::repositories::Repositories;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DefaultRepositoriesError {
    #[error(transparent)]
    PostgresqlError(#[from] PostgresqlError),
}

pub struct DefaultRepositories {
    pg_user_repository: PgUserRepository,
}

impl DefaultRepositories {
    pub async fn new(postgres_url: &str) -> Result<Self, DefaultRepositoriesError> {
        let postgresql = Postgresql::new(postgres_url).await?;

        Ok(Self {
            pg_user_repository: PgUserRepository::new(postgresql),
        })
    }
}

impl Repositories for DefaultRepositories {
    type UserRepositoryImpl = PgUserRepository;

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.pg_user_repository
    }
}
