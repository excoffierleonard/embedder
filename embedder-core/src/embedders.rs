//! Different embedders

use crate::errors::EmbedderError;

mod ollama;
mod openai;

pub use openai::OpenAIClient;

pub trait EmbeddingClient {
    async fn create_embeddings(self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, EmbedderError>;
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
