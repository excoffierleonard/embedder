use dotenv::dotenv;
use embedder_core::{InputTexts, OllamaClient, OpenAIClient};
use std::env::var;

// Test the Ollama embedding client flow
#[tokio::test]
async fn embed_ollama_success() {
    dotenv().ok();

    let texts = vec!["Hello, world!".to_string(), "Goodbye, world!".to_string()];
    let client = OllamaClient::default();

    let embeddings = InputTexts::new(texts.clone()).embed(client).await.unwrap();

    // Check that the embeddings are returned
    assert_eq!(embeddings.len(), texts.len());
}

// Test the OpenAI embedding client flow
#[tokio::test]
async fn embed_openai_success() {
    dotenv().ok();
    let api_key = var("OPENAI_API_KEY").unwrap().to_string();

    let texts = vec!["Hello, world!".to_string(), "Goodbye, world!".to_string()];
    let client = OpenAIClient::new(api_key, None);

    let embeddings = InputTexts::new(texts.clone()).embed(client).await.unwrap();

    // Check that the embeddings are returned
    assert_eq!(embeddings.len(), texts.len());
}
