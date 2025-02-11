//! Different embedders

use crate::errors::EmbedderError;
use std::future::Future;

mod ollama;
mod openai;

pub use ollama::{OllamaClient, DEFAULT_OLLAMA_EMBEDDING_MODEL};
pub use openai::OpenAIClient;

/// Embedding client trait
pub trait EmbeddingClient {
    fn create_embeddings(
        self,
        texts: Vec<String>,
    ) -> impl Future<Output = Result<Vec<Vec<f32>>, EmbedderError>> + Send;
}

/// Input texts to be embedded
pub struct InputTexts(Vec<String>);

impl InputTexts {
    /// Create a new instance of InputTexts
    pub fn new(texts: Vec<String>) -> Self {
        Self(texts)
    }

    /// Embed the input texts
    pub async fn embed<T: EmbeddingClient>(
        self,
        client: T,
    ) -> Result<Vec<Vec<f32>>, EmbedderError> {
        client.create_embeddings(self.0).await
    }
}
