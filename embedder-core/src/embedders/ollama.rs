//! Ollama embedder

use crate::{embedders::EmbeddingClient, errors::EmbedderError};
use serde::{Deserialize, Serialize};

const OLLAMA_API_URL: &str = "http://localhost:11434/api/embed";
const OLLAMA_EMBEDDING_MODEL: &str = "nomic-embed-text";

pub struct OllamaClient {
    client: reqwest::Client,
}

impl OllamaClient {
    /// Create a new instance of OllamaClient
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
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
            model: OLLAMA_EMBEDDING_MODEL.to_string(),
        };

        let response: OllamaResponse = self
            .client
            .post(OLLAMA_API_URL)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response.embeddings)
    }
}
