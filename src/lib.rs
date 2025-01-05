use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env::var, error::Error};

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
pub async fn embed(text: Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    dotenv().ok();

    let client = Client::new();

    let response = client
        .post("https://api.openai.com/v1/embeddings")
        .bearer_auth(var("OPENAI_API_KEY")?)
        .json(&RequestBody {
            input: text,
            model: "text-embedding-3-large".to_string(),
        })
        .send()
        .await?
        .error_for_status()?;

    let embedding: EmbeddingResponse = response.json().await?;

    Ok(embedding.data.into_iter().map(|e| e.embedding).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn embed_success() {
        let texts = vec!["The skye is blue", "The sun is shining"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result = embed(texts).await.unwrap();

        assert_eq!(result[0].len(), 3072);
        assert_eq!(result[1].len(), 3072);
    }
}
