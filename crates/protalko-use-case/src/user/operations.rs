use protalko_domain::{
    entities::user::{User, UserId, UserWithoutId},
    repositories::{
        user::{UserRepository, UserRepositoryError},
        Repositories,
    },
    value_objects::{
        error::{ErrorModel, IntoErrorModel},
        response_status::Status,
    },
};

use crate::user::UserUseCase;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserUseCaseError {
    #[error(transparent)]
    UserRepositoryError(#[from] UserRepositoryError),
    #[error("User Not Found (0:?)")]
    UserNotFoundError(UserId),
}

impl IntoErrorModel for UserUseCaseError {
    fn into_error_model(self) -> ErrorModel {
        match self {
            Self::UserNotFoundError(e) => ErrorModel::new(
                Status::NotFound,
                format!("user/not-found:{}", e.value().hyphenated()).to_string(),
            ),
            Self::UserRepositoryError(_) => {
                ErrorModel::new(Status::InternalServerError, "user/repository".to_string())
            }
        }
    }
}

impl<R: Repositories> UserUseCase<R> {
    pub async fn create(&self, user: UserWithoutId) -> Result<UserId, UserUseCaseError> {
        let res = self.repositories.user_repository().create(user).await;

        match res {
            Ok(id) => Ok(id),
            Err(e) => Err(UserUseCaseError::UserRepositoryError(e)),
        }
    }

    pub async fn update(&self, user: User) -> Result<(), UserUseCaseError> {
        let res = self.repositories.user_repository().update(user).await;

        match res {
            Ok(()) => Ok(()),
            Err(e) => Err(UserUseCaseError::UserRepositoryError(e)),
        }
    }

    pub async fn get_by_id(&self, id: UserId) -> Result<User, UserUseCaseError> {
        let res = self
            .repositories
            .user_repository()
            .get_by_id(id.clone())
            .await;

        match res {
            Ok(optional) => match optional {
                Some(u) => Ok(u),
                None => Err(UserUseCaseError::UserNotFoundError(id)),
            },
            Err(e) => Err(UserUseCaseError::UserRepositoryError(e)),
        }
    }

    pub async fn delete_by_id(&self, id: UserId) -> Result<(), UserUseCaseError> {
        let res = self.repositories.user_repository().delete_by_id(id).await;

        match res {
            Ok(()) => Ok(()),
            Err(e) => Err(UserUseCaseError::UserRepositoryError(e)),
        }
    }
}
