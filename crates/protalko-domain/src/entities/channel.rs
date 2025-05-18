use crate::value_object;
use getset::Getters;
use uuid::Uuid;

#[derive(Clone, Getters)]
pub struct Channel {
    #[getset(get = "pub")]
    id: ChannelId,
    #[getset(get = "pub")]
    name: ChannelName,
    // TODO:
    // #[getset(get = "pub")]
    // visible_to: Vec<CommunityRoleId>,
}

value_object!(ChannelId(Uuid));
value_object!(ChannelName(String));
