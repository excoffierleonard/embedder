use crate::errors::EmbeddingError;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Serialize)]
struct RequestBody {
    input: Vec<String>,
    model: String,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<Embedding>,
}

#[derive(Deserialize)]
struct Embedding {
    embedding: Vec<f32>,
}

/// Embeds the given text using the OpenAI API.
pub async fn embed(text: Vec<String>) -> Result<Vec<Vec<f32>>, EmbeddingError> {
    dotenv().ok();

    let url = "https://api.openai.com/v1/embeddings";

    let auth_token = var("OPENAI_API_KEY")?;

    let request_body = RequestBody {
        input: text,
        model: "text-embedding-3-large".to_string(),
    };

    let client = Client::new();

    let response: EmbeddingResponse = client
        .post(url)
        .bearer_auth(auth_token)
        .json(&request_body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let embeddings = response.data.into_iter().map(|e| e.embedding).collect();

    Ok(embeddings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn embed_success() {
        let input_texts = vec!["The sky is blue", "The sun is shining"];
        let texts = input_texts.iter().map(|s| s.to_string()).collect();
        let result = embed(texts).await.unwrap();

        assert_eq!(result.len(), input_texts.len());
        assert_eq!(result[0].len(), 3072);
        assert_eq!(result[1].len(), 3072);
    }
}