use crate::value_object;
use getset::Getters;
use uuid::Uuid;

#[derive(Clone, Getters)]
pub struct User {
    #[getset(get = "pub")]
    id: UserId,
    #[getset(get = "pub")]
    visible_id: UserVisibleId,
    #[getset(get = "pub")]
    name: UserName,
}

impl User {
    pub fn new(id: UserId, visible_id: UserVisibleId, name: UserName) -> Self {
        Self {
            id,
            visible_id,
            name,
        }
    }
}

value_object!(UserId(Uuid));
value_object!(UserVisibleId(String));
value_object!(UserName(String));
