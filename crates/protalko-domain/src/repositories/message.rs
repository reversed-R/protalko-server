use crate::entities::{
    channel::ChannelId,
    community::CommunityId,
    message::{Message, MessageId},
};

use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum MessageRepositoryError {}

#[allow(async_fn_in_trait)]
pub trait MessageRepository: Send + Sync + 'static {
    async fn create(
        &self,
        community_id: CommunityId,
        channel_id: ChannelId,
        message: Message,
    ) -> Result<Option<MessageId>, MessageRepositoryError>;

    async fn update(
        &self,
        community_id: CommunityId,
        channel_id: ChannelId,
        message: Message,
    ) -> Result<(), MessageRepositoryError>;

    async fn get_by_id(
        &self,
        community_id: CommunityId,
        channel_id: ChannelId,
        id: MessageId,
    ) -> Result<Option<Message>, MessageRepositoryError>;

    async fn list(
        &self,
        community_id: CommunityId,
        channel_id: ChannelId,
    ) -> Result<Vec<Message>, MessageRepositoryError>;

    async fn delete_by_id(
        &self,
        community_id: CommunityId,
        channel_id: ChannelId,
        id: MessageId,
    ) -> Result<(), MessageRepositoryError>;
}
