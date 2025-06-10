use crate::{
    models::{
        error::AppError,
        user::{CreateUserPresentation, CreatedUserPresentation, UserPresentation},
    },
    modules::Modules,
};
use protalko_domain::{
    entities::user::{UserId, UserWithoutId},
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

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    operation_id = "getUserById",
    tag = "users",
    responses(
        (status = 200, description = "OK", body = UserPresentation),
        (status = 400, description = "Bad Request", body = crate::models::error::ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = crate::models::error::ErrorResponse),
        (status = 500, description = "Internal Server Error", body = crate::models::error::ErrorResponse),
    ),
    security(()),
)]
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

#[utoipa::path(
    post,
    path = "/users",
    operation_id = "postUser",
    tag = "users",
    responses(
        (status = 201, description = "Created", body = CreatedUserPresentation),
        (status = 400, description = "Bad Request", body = crate::models::error::ErrorResponse),
        (status = 422, description = "Unprocessable Entity", body = crate::models::error::ErrorResponse),
        (status = 500, description = "Internal Server Error", body = crate::models::error::ErrorResponse),
    ),
    security(()),
)]
pub async fn handle_post(
    State(modules): State<Arc<Modules<DefaultRepositories>>>,
    Json(raw_user): Json<CreateUserPresentation>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserWithoutId::from(raw_user);

    let res = modules.user_use_case().create(user).await;

    match res {
        Ok(id) => Ok(Json(CreatedUserPresentation::from(id))),
        Err(e) => Err(AppError::from(e.into_error_model())),
    }
}
