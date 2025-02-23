//! Error types and implementations for the crate.

#[derive(Debug)]
pub enum EmbedderError {
    /// An error occurred while interacting with an external HTTP api.
    HTTPError(reqwest::Error),
}

impl std::error::Error for EmbedderError {}

impl std::fmt::Display for EmbedderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbedderError::HTTPError(e) => write!(f, "HTTP error: {}", e),
        }
    }
}

impl From<reqwest::Error> for EmbedderError {
    fn from(err: reqwest::Error) -> Self {
        EmbedderError::HTTPError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::get;

    // Test each error variant's Display implementation
    #[tokio::test]
    async fn error_display() {
        let http_err = EmbedderError::HTTPError(get("invalid-url").await.unwrap_err());

        assert!(http_err.to_string().starts_with("HTTP error:"));
    }
}
