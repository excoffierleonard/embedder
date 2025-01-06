use crate::errors::EmbeddingError;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query, QueryBuilder, Row};
use std::env::var;

const EMBEDDING_DIMENSION: usize = 3072;
const OPENAI_API_URL: &str = "https://api.openai.com/v1/embeddings";
const EMBEDDING_MODEL: &str = "text-embedding-3-large";

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

/// Embeds the given text using the OpenAI API.
pub async fn embed(text: Vec<String>) -> Result<Vec<Vec<f32>>, EmbeddingError> {
    dotenv().ok();

    let auth_token = var("OPENAI_API_KEY")?;

    let request = OpenAIRequest {
        input: text,
        model: EMBEDDING_MODEL.to_string(),
    };

    let client = Client::new();

    let response: OpenAIResponse = client
        .post(OPENAI_API_URL)
        .bearer_auth(auth_token)
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(response.data.into_iter().map(|e| e.embedding).collect())
}

pub async fn initialize() -> Result<(), EmbeddingError> {
    dotenv().ok();

    let database_url = var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    // Create the vector extension for the database
    query(
        "
        CREATE EXTENSION IF NOT EXISTS vector;
    ",
    )
    .execute(&pool)
    .await?;

    // Create the table to store the embeddings
    query(
        "
        CREATE TABLE IF NOT EXISTS embeddings (
            id UUID DEFAULT gen_random_uuid(),
            text TEXT,
            embedding vector($1),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (id)
        );
    ",
    )
    .bind(EMBEDDING_DIMENSION as i32)
    .execute(&pool)
    .await?;

    Ok(())
}

pub struct EmbeddingRecord {
    pub text: String,
    pub embedding: Vec<f32>,
}

pub async fn store(records: Vec<EmbeddingRecord>) -> Result<(), EmbeddingError> {
    dotenv().ok();

    let database_url = var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let mut query_builder = QueryBuilder::new(
        "
        INSERT INTO embeddings (text, embedding)
    ",
    );

    query_builder.push_values(records, |mut b, record| {
        b.push_bind(record.text).push_bind(record.embedding);
    });

    query_builder.build().execute(&pool).await?;

    Ok(())
}

pub async fn fetch_similar(embedding: Vec<f32>, top_k: i32) -> Result<Vec<String>, EmbeddingError> {
    dotenv().ok();

    let database_url = var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    // TODO: Need to find better way to build query than using format
    // Find the most similar embeddings
    let result = query(&format!(
        "
        SELECT text, embedding <=> $1::vector({}) AS distance
        FROM embeddings
        WHERE embedding IS NOT NULL
        ORDER BY distance ASC
        LIMIT $2;
    ",
        EMBEDDING_DIMENSION
    ))
    .bind(embedding)
    .bind(top_k)
    .fetch_all(&pool)
    .await?;

    let texts = result
        .iter()
        .map(|row| row.get("text"))
        .collect::<Vec<String>>();

    Ok(texts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    fn generate_random_text() -> String {
        format!("The sky is blue {}", random::<u32>())
    }

    fn generate_random_embedding() -> Vec<f32> {
        (0..EMBEDDING_DIMENSION)
            .map(|_| random::<f32>() * 2.0 - 1.0)
            .collect()
    }

    #[tokio::test]
    async fn embed_success() {
        let texts = vec![generate_random_text(), generate_random_text()];
        let result = embed(texts.clone()).await.unwrap();

        assert_eq!(result.len(), texts.len());
        assert_eq!(result[0].len(), EMBEDDING_DIMENSION);
        assert_eq!(result[1].len(), EMBEDDING_DIMENSION);
    }

    // TODO: Make these tests not against prod db. And also the other test not depend on that one since they may run in parralel.

    #[tokio::test]
    async fn test_initialize_success() {
        dotenv().ok();
        initialize().await.unwrap();

        let database_url = var("DATABASE_URL").unwrap();
        let pool = PgPool::connect(&database_url).await.unwrap();

        // Check that the vector extension was created
        let result = query(
            "
                SELECT * FROM pg_extension WHERE extname = 'vector';
            ",
        )
        .fetch_one(&pool)
        .await;
        assert!(result.is_ok());

        // Check that the embedding table was created
        let result = query(
            "
                SELECT * FROM information_schema.tables WHERE table_name = 'embeddings';
            ",
        )
        .fetch_one(&pool)
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_store_success() {
        dotenv().ok();
        initialize().await.unwrap();

        let database_url = var("DATABASE_URL").unwrap();
        let pool = PgPool::connect(&database_url).await.unwrap();

        let text = generate_random_text();
        let embedding = generate_random_embedding();

        // Create a vector with a single EmbeddingRecord
        let records = vec![EmbeddingRecord {
            text: text.clone(),
            embedding: embedding.clone(),
        }];

        store(records).await.unwrap();

        // Check that the row was inserted
        let result = query(
            "
                SELECT * FROM embeddings WHERE text = $1;
            ",
        )
        .bind(text)
        .fetch_one(&pool)
        .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_similar_success() {
        initialize().await.unwrap();

        let text = generate_random_text();
        let embedding = generate_random_embedding();

        // Create a vector with a single EmbeddingRecord
        let records = vec![EmbeddingRecord {
            text: text.clone(),
            embedding: embedding.clone(),
        }];

        store(records).await.unwrap();

        let top_k = 5;

        let similar_texts = fetch_similar(embedding.clone(), top_k.clone())
            .await
            .unwrap();

        assert_eq!(similar_texts.len(), top_k as usize);
        assert_eq!(similar_texts[0], text);
    }
}
