use serde::Serialize;
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

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<ApiError> for Result<Response, worker::Error> {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::NotFound(msg) => json_with_status(&ErrorResponse { error: msg }, 404),
            ApiError::InternalServerError(msg) => {
                json_with_status(&ErrorResponse { error: msg }, 500)
            }
        }
    }
}

fn json_with_status<T: Serialize>(value: &T, status: u16) -> Result<Response, worker::Error> {
    let mut resp = Response::from_json(value)?;
    resp = resp.with_status(status);
    Ok(resp)
}
