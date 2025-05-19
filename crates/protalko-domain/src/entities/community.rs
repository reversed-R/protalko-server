use crate::{entities::community_member::CommunityMember, value_object};
use getset::Getters;
use uuid::Uuid;

#[derive(Clone, Getters)]
pub struct Community {
    #[getset(get = "pub")]
    id: CommunityId,
    #[getset(get = "pub")]
    name: CommunityName,

    #[getset(get = "pub")]
    members: Vec<CommunityMember>,
    // TODO:
    // #[getset(get = "pub")]
    // roles: Vec<CommunityRole>,

    // TODO:
    // #[getset(get = "pub")]
    // stickers: Vec<CommunitySticker>,
}

value_object!(CommunityId(Uuid));
value_object!(CommunityName(String));
