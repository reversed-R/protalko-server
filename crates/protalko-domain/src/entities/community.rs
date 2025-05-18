use crate::{
    entities::user::{UserId, UserName},
    value_object,
};
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

#[derive(Clone, Getters)]
pub struct CommunityMember {
    #[getset(get = "pub")]
    id: UserId,
    #[getset(get = "pub")]
    name: UserName,
    // TODO:
    // #[getset(get = "pub")]
    // roles: Vec<CommunityRoleId>,
}
