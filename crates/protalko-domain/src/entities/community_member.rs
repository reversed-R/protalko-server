use crate::entities::user::{UserId, UserName};
use getset::Getters;

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
