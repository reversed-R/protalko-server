#[derive(Clone)]
pub enum Status {
    Ok,
    Created,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
}
