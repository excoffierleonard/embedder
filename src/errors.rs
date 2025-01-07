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
    /// The input text is empty.
    EmptyInput,
    /// Invalid dimensions submitted for the creation of a new embedding.
    InvalidDimension { expected: usize, got: usize },
    /// The Embedding and InputTexts have mismatched lengths.
    MismatchedLength { texts: usize, embeddings: usize },
    /// The input text is an empty string.
    EmptyString,
    /// The embedding vector contains invalid values.
    InvalidValues,
}

impl std::error::Error for EmbeddingError {}

impl std::fmt::Display for EmbeddingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmbeddingError::ApiError(e) => write!(f, "API error: {}", e),
            EmbeddingError::EnvError(e) => write!(f, "Environment variable error: {}", e),
            EmbeddingError::DbError(e) => write!(f, "Database error: {}", e),
            EmbeddingError::EmptyInput => write!(f, "Input text is empty"),
            EmbeddingError::InvalidDimension { expected, got } => {
                write!(f, "Invalid dimension: expected {}, got {}", expected, got)
            }
            EmbeddingError::MismatchedLength { texts, embeddings } => {
                write!(
                    f,
                    "Mismatched lengths: texts={}, embeddings={}",
                    texts, embeddings
                )
            }
            EmbeddingError::EmptyString => write!(f, "Input text is an empty string"),
            EmbeddingError::InvalidValues => write!(f, "Invalid values in the embedding vector"),
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

impl From<dotenv::Error> for EmbeddingError {
    fn from(_: dotenv::Error) -> Self {
        EmbeddingError::EnvError(std::env::VarError::NotPresent)
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
        let empty_input_err = EmbeddingError::EmptyInput;
        let invalid_dim_err = EmbeddingError::InvalidDimension {
            expected: 3072,
            got: 1024,
        };
        let invalid_values_err = EmbeddingError::InvalidValues;
        let mismatched_len_err = EmbeddingError::MismatchedLength {
            texts: 3,
            embeddings: 2,
        };
        let empty_string_err = EmbeddingError::EmptyString;

        assert!(api_err.to_string().starts_with("API error:"));
        assert!(env_err
            .to_string()
            .starts_with("Environment variable error:"));
        assert!(db_err.to_string().starts_with("Database error:"));
        assert_eq!(empty_input_err.to_string(), "Input text is empty");
        assert_eq!(
            invalid_dim_err.to_string(),
            "Invalid dimension: expected 3072, got 1024"
        );
        assert_eq!(
            invalid_values_err.to_string(),
            "Invalid values in the embedding vector"
        );
        assert_eq!(
            mismatched_len_err.to_string(),
            "Mismatched lengths: texts=3, embeddings=2"
        );
        assert_eq!(
            empty_string_err.to_string(),
            "Input text is an empty string"
        );
    }
}
