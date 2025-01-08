use dotenv::dotenv;
use embedder_core::{InputTexts, OpenAIClient};
use std::env::var;

#[test]
fn embed_ollama_success() {}

#[tokio::test]
async fn embed_openai_success() {
    dotenv().ok();
    let texts = vec!["Hello, world!".to_string(), "Goodbye, world!".to_string()];

    let api_key = var("OPENAI_API_KEY").unwrap().to_string();

    let client = OpenAIClient::new(api_key);

    let embeddings = InputTexts::new(texts).embed(client).await.unwrap();

    assert_eq!(embeddings.len(), 2);
}
