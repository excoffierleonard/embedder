//! OpenAI embedder

use crate::{embedders::EmbeddingClient, errors::EmbedderError};
use serde::{Deserialize, Serialize};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/embeddings";
const OPENAI_EMBEDDING_MODEL: &str = "text-embedding-3-large";

/// OpenAI Client
pub struct OpenAIClient {
    api_key: String,
    model: String,
    base_url: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    /// Create a new instance of OpenAIClient
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            model: OPENAI_EMBEDDING_MODEL.to_string(),
            base_url: OPENAI_API_URL.to_string(),
            client: reqwest::Client::new(),
        }
    }
}

impl EmbeddingClient for OpenAIClient {
    async fn create_embeddings(self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, EmbedderError> {
        #[derive(Serialize)]
        struct OpenAIRequest {
            input: Vec<String>,
            model: String,
        }

        #[derive(Deserialize)]
        struct OpenAIResponse {
            data: Vec<OpenAIEmbedding>,
        }

        #[derive(Deserialize)]
        struct OpenAIEmbedding {
            embedding: Vec<f32>,
        }

        let request = OpenAIRequest {
            input: texts,
            model: self.model,
        };

        let response: OpenAIResponse = self
            .client
            .post(&self.base_url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response.data.into_iter().map(|d| d.embedding).collect())
    }
}
