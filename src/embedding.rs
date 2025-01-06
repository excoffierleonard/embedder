use crate::errors::EmbeddingError;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query, QueryBuilder, Row};
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
            embedding vector(3072),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (id)
        );
    ",
    )
    .execute(&pool)
    .await?;

    Ok(())
}

#[derive(Debug)]
pub struct EmbeddingRecord {
    text: String,
    embedding: Vec<f32>,
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

    // Find the most similar embeddings
    let result = query(
        "
        SELECT text, embedding <=> $1::vector(3072) AS distance
        FROM embeddings
        WHERE embedding IS NOT NULL
        ORDER BY distance ASC
        LIMIT $2;
    ",
    )
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

    #[tokio::test]
    async fn embed_success() {
        let input_texts = vec!["The sky is blue", "The sun is shining"];
        let texts = input_texts.iter().map(|s| s.to_string()).collect();
        let result = embed(texts).await.unwrap();

        assert_eq!(result.len(), input_texts.len());
        assert_eq!(result[0].len(), 3072);
        assert_eq!(result[1].len(), 3072);
    }

    // TODO: Make this test not against prod db. And also the other test not depend on that one since they may run in parralel.
    #[tokio::test]
    async fn test_initialize_success() {
        dotenv().ok();

        let database_url = var("DATABASE_URL").unwrap();

        let pool = PgPool::connect(&database_url).await.unwrap();

        initialize().await.unwrap();

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

        let database_url = var("DATABASE_URL").unwrap();
        let pool = PgPool::connect(&database_url).await.unwrap();

        initialize().await.unwrap();

        let text = format!("The sky is blue {}", random::<u32>());
        let embedding = (0..3072)
            .map(|_| random::<f32>() * 2.0 - 1.0)
            .collect::<Vec<f32>>();

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

        let text = format!("The sky is blue {}", random::<u32>());
        let embedding = (0..3072)
            .map(|_| random::<f32>() * 2.0 - 1.0)
            .collect::<Vec<f32>>();

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
