//! Ollama embedder

use crate::{embedders::EmbeddingClient, errors::EmbedderError};
use serde::{Deserialize, Serialize};

const DEFAULT_OLLAMA_API_URL: &str = "http://localhost:11434/api/embed";
const DEFAULT_OLLAMA_EMBEDDING_MODEL: &str = "nomic-embed-text";

/// Ollama Client
pub struct OllamaClient {
    model: String,
    base_url: String,
    client: reqwest::Client,
}

impl OllamaClient {
    /// Create a new instance of OllamaClient
    pub fn new(model: &str, base_url: &str) -> Self {
        Self {
            model: model.to_string(),
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }
}

impl Default for OllamaClient {
    /// Create a new instance of OllamaClient with default values
    fn default() -> Self {
        Self::new(DEFAULT_OLLAMA_EMBEDDING_MODEL, DEFAULT_OLLAMA_API_URL)
    }
}

impl EmbeddingClient for OllamaClient {
    async fn create_embeddings(self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, EmbedderError> {
        #[derive(Serialize)]
        struct OllamaRequest {
            input: Vec<String>,
            model: String,
        }

        #[derive(Deserialize)]
        struct OllamaResponse {
            embeddings: Vec<Vec<f32>>,
        }

        let request = OllamaRequest {
            input: texts,
            model: self.model,
        };

        let response: OllamaResponse = self
            .client
            .post(&self.base_url)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response.embeddings)
    }
}
