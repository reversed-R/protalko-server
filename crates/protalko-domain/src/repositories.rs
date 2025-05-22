pub mod channel;
pub mod community;
pub mod community_member;
pub mod message;
pub mod user;

pub trait Repositories: Send + Sync + 'static {
    type UserRepositoryImpl: user::UserRepository;

    fn user_repository(&self) -> &Self::UserRepositoryImpl;
}
