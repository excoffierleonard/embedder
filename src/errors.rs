//! Error types and implementations for the crate.

/// Custom error type for the crate.
#[derive(Debug)]
pub enum EmbeddingError {
    /// An error occurred while interacting with the OpenAI API.
    ApiError(reqwest::Error),
    /// An error occurred while interacting with the environment.
    EnvError(std::env::VarError),
    /// Database error
    DbError(sqlx::Error),
}

impl std::error::Error for EmbeddingError {}

impl std::fmt::Display for EmbeddingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbeddingError::ApiError(e) => write!(f, "API error: {}", e),
            EmbeddingError::EnvError(e) => write!(f, "Environment variable error: {}", e),
            EmbeddingError::DbError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl From<reqwest::Error> for EmbeddingError {
    fn from(err: reqwest::Error) -> Self {
        EmbeddingError::ApiError(err)
    }
}

impl From<std::env::VarError> for EmbeddingError {
    fn from(err: std::env::VarError) -> Self {
        EmbeddingError::EnvError(err)
    }
}

impl From<sqlx::Error> for EmbeddingError {
    fn from(err: sqlx::Error) -> Self {
        EmbeddingError::DbError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::get;

    #[tokio::test]
    async fn test_error_display() {
        // Test each error variant's Display implementation
        let api_err = EmbeddingError::ApiError(get("invalid-url").await.unwrap_err());
        let env_err = EmbeddingError::EnvError(std::env::VarError::NotPresent);
        let db_err = EmbeddingError::DbError(sqlx::Error::RowNotFound);
        

        assert!(api_err.to_string().starts_with("API error:"));
        assert!(env_err
            .to_string()
            .starts_with("Environment variable error:"));
        assert!(db_err.to_string().starts_with("Database error:"));
    }
}
