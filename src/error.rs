use worker::Response;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    InternalServerError(String),
}

impl From<worker::Error> for ApiError {
    fn from(e: worker::Error) -> Self {
        ApiError::InternalServerError(e.to_string())
    }
}

impl From<ApiError> for Result<Response, worker::Error> {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::NotFound(msg) => Response::error(msg, 404),
            ApiError::InternalServerError(msg) => Response::error(msg, 500),
        }
    }
}
