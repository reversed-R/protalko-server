use crate::entities::{
    channel::{Channel, ChannelId},
    community::CommunityId,
};

use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum ChannelRepositoryError {}

#[allow(async_fn_in_trait)]
pub trait ChannelRepository: Send + Sync + 'static {
    async fn create(
        &self,
        community_id: CommunityId,
        channel: Channel,
    ) -> Result<Option<ChannelId>, ChannelRepositoryError>;

    async fn update(
        &self,
        community_id: CommunityId,
        channel: Channel,
    ) -> Result<(), ChannelRepositoryError>;

    async fn get_by_id(
        &self,
        community_id: CommunityId,
        id: ChannelId,
    ) -> Result<Option<Channel>, ChannelRepositoryError>;

    async fn list(&self, community_id: CommunityId)
        -> Result<Vec<Channel>, ChannelRepositoryError>;

    async fn delete_by_id(
        &self,
        community_id: CommunityId,
        id: ChannelId,
    ) -> Result<(), ChannelRepositoryError>;
}
