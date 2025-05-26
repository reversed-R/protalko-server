#[utoipa::path(
    get,
    path = "/health",
    operation_id = "health",
    tag = "health",
    responses(
        (status = 200, description = "OK", body = String),
        (status = 500, description = "Internal Server Error", body = crate::models::error::ErrorResponse),
    ),
    security(()),
)]
pub async fn handle_health() -> String {
    "OK".to_string()
}
