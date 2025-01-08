//! Error types and implementations for the crate.

#[derive(Debug)]
pub enum EmbedderError {
    /// An error occurred while interacting with an external api API.
    ApiError(reqwest::Error),
}

impl std::error::Error for EmbedderError {}

impl std::fmt::Display for EmbedderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbedderError::ApiError(e) => write!(f, "API error: {}", e),
        }
    }
}

impl From<reqwest::Error> for EmbedderError {
    fn from(err: reqwest::Error) -> Self {
        EmbedderError::ApiError(err)
    }
}
