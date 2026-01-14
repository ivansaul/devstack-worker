#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error while reading file")]
    IoError(#[from] std::io::Error),

    #[error("Parsing error: {0}")]
    ParseError(String),

    #[error("Frontmatter parsing error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
