use crate::entities::{
    community::{CommunityId, CommunityMember},
    user::UserId,
};

use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum CommunityMemberRepositoryError {}

#[allow(async_fn_in_trait)]
pub trait CommunityMemberRepository: Send + Sync + 'static {
    async fn create(
        &self,
        community_id: CommunityId,
        community_member: CommunityMember,
    ) -> Result<(), CommunityMemberRepositoryError>;

    async fn update(
        &self,
        community_id: CommunityId,
        community_member: CommunityMember,
    ) -> Result<(), CommunityMemberRepositoryError>;

    async fn list(
        &self,
        community_id: CommunityId,
    ) -> Result<Vec<CommunityMember>, CommunityMemberRepositoryError>;

    async fn delete_by_id(
        &self,
        community_id: CommunityId,
        user_id: UserId,
    ) -> Result<(), CommunityMemberRepositoryError>;
}
