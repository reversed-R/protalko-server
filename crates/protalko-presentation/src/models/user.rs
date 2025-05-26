use protalko_domain::entities::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateUserPresentation {
    pub visible_id: String,
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserPresentation {
    pub visible_id: String,
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserPresentation {
    pub id: String,
    pub visible_id: String,
    pub name: String,
}

impl From<User> for UserPresentation {
    fn from(value: User) -> Self {
        Self {
            id: value.id().clone().value().as_hyphenated().to_string(),
            visible_id: value.visible_id().clone().value().to_string(),
            name: value.name().clone().value().to_string(),
        }
    }
}
