use crate::errors::EmbeddingError;
use dotenv::dotenv;
use sqlx::{postgres::PgPool, query, Row};
use std::env::var;

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

// TODO: This function need to accept multiple inputs for efficient batch processing
pub async fn store(text: String, embedding: Vec<f32>) -> Result<(), EmbeddingError> {
    dotenv().ok();

    let database_url = var("DATABASE_URL")?;

    let pool = PgPool::connect(&database_url).await?;

    // Insert rows
    query(
        "
        INSERT INTO embeddings (text, embedding)
        VALUES ($1, $2);
    ",
    )
    .bind(text)
    .bind(embedding)
    .execute(&pool)
    .await?;

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

        store(text.clone(), embedding.clone()).await.unwrap();

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

        store(text.clone(), embedding.clone()).await.unwrap();

        let top_k = 5;

        let similar_texts = fetch_similar(embedding.clone(), top_k.clone())
            .await
            .unwrap();

        assert_eq!(similar_texts.len(), top_k as usize);
        assert_eq!(similar_texts[0], text);
    }
}
