#[derive(Clone)]
pub enum Status {
    Ok,
    Created,
    Unauthorized,
    Forbidden,
    NotFound,
}

impl Status {
    pub fn code(&self) -> u32 {
        match &self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
        }
    }
}
