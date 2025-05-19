use crate::entities::user::{User, UserId};

use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum UserRepositoryError {}

#[allow(async_fn_in_trait)]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: User) -> Result<Option<UserId>, UserRepositoryError>;

    async fn update(&self, user: User) -> Result<(), UserRepositoryError>;

    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, UserRepositoryError>;

    async fn delete_by_id(&self, id: UserId) -> Result<(), UserRepositoryError>;
}
