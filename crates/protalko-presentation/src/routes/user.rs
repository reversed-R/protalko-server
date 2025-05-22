use crate::{
    models::{error::AppError, user::UserPresentation},
    modules::Modules,
};
use protalko_domain::{
    entities::user::UserId,
    value_objects::{
        error::{ErrorModel, IntoErrorModel},
        response_status::Status,
    },
};
use protalko_infrastructure::shared::DefaultRepositories;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn handle_get_by_id(
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
    Path(raw_user_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = UserId::new(Uuid::parse_str(&raw_user_id).map_err(|_| {
        AppError::from(ErrorModel::new(
            Status::BadRequest,
            "user/invalid-uuid".to_string(),
        ))
    })?);

    let res = modules.user_use_case().get_by_id(user_id).await;

    match res {
        Ok(u) => Ok(Json(UserPresentation::from(u))),
        Err(e) => Err(AppError::from(e.into_error_model())),
    }
}
