use crate::{entities::user::UserId, value_object, value_objects::datetime::DateTime};
use getset::Getters;
use uuid::Uuid;

#[derive(Clone, Getters)]
pub struct Message {
    // key
    #[getset(get = "pub")]
    id: MessageId,

    // content
    #[getset(get = "pub")]
    text: MessageText,

    // TODO:
    // #[getset(get = "pub")]
    // file: Option<FileId>,

    // TODO:
    // #[getset(get = "pub")]
    // reactions: Vec<StickerId>,
    #[getset(get = "pub")]
    replies_to: Option<MessageId>,

    // TODO:
    // #[getset(get = "pub")]
    // thread: Option<ThreadId>,

    // meta data
    #[getset(get = "pub")]
    created_by: UserId,
    #[getset(get = "pub")]
    posted_by: UserId,

    #[getset(get = "pub")]
    created_at: DateTime,
    #[getset(get = "pub")]
    updated_at: DateTime,
}

value_object!(MessageId(Uuid));
value_object!(MessageText(String));
