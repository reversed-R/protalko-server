use super::response_status::Status;
use getset::Getters;

#[derive(Clone, Getters)]
pub struct ErrorModel {
    #[getset(get = "pub")]
    status: Status,
    #[getset(get = "pub")]
    message: String,
}

impl ErrorModel {
    pub fn new(status: Status, message: String) -> Self {
        Self { status, message }
    }
}

pub trait IntoErrorModel {
    fn into_error_model(self) -> ErrorModel;
}
