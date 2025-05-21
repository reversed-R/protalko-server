use protalko_domain::{
    entities::user::{User, UserId},
    repositories::{
        user::{UserRepository, UserRepositoryError},
        Repositories,
    },
};

use crate::user::UserUseCase;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
}

impl<R: Repositories> UserUseCase<R> {
    pub async fn create(&self, user: User) -> Result<UserId, UserUseCaseError> {
        let res = self.repositories.user_repository().create(user).await;

        match res {
            Ok(id) => Ok(id),
            Err(e) => Err(UserUseCaseError::UserRepositoryError(e)),
        }
    }
}

