use protalko_domain::entities::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserPresentation {
    visible_id: String,
    name: String,
}

#[derive(Deserialize)]
pub struct GetUserPresentation {
    id: String,
}

#[derive(Deserialize)]
pub struct UpdateUserPresentation {
    id: String,
    visible_id: String,
    name: String,
}

#[derive(Deserialize)]
pub struct DeleteUserPresentation {
    id: String,
}

#[derive(Serialize)]
pub struct UserPresentation {
    id: String,
    visible_id: String,
    name: String,
}

impl From<User> for UserPresentation {
    fn from(value: User) -> Self {
        Self {
            id: value.id().clone().value().as_hyphenated().to_string(),
            visible_id: value.visible_id().clone().value(),
            name: value.name().value(),
        }
    }
}
