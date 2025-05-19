use crate::entities::community::{Community, CommunityId};

use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum CommunityRepositoryError {}

#[allow(async_fn_in_trait)]
pub trait CommunityRepository: Send + Sync + 'static {
    async fn create(
        &self,
        community: Community,
    ) -> Result<Option<CommunityId>, CommunityRepositoryError>;

    async fn update(&self, community: Community) -> Result<(), CommunityRepositoryError>;

    async fn get_by_id(
        &self,
        id: CommunityId,
    ) -> Result<Option<Community>, CommunityRepositoryError>;

    async fn delete_by_id(&self, id: CommunityId) -> Result<(), CommunityRepositoryError>;
}
