pub mod postgresql;

use crate::{shared::postgresql::Postgresql, user::PgUserRepository};
use protalko_domain::repositories::Repositories;

pub struct DefaultRepositories {
    pg_user_repository: PgUserRepository,
}

impl DefaultRepositories {
    pub fn new(postgresql: Postgresql) -> Self {
        Self {
            pg_user_repository: PgUserRepository::new(postgresql),
        }
    }
}

impl Repositories for DefaultRepositories {
    type UserRepositoryImpl = PgUserRepository;

    fn user_repository(&self) -> &Self::UserRepositoryImpl {
        &self.pg_user_repository
    }
}
