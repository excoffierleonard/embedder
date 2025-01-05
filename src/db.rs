use crate::errors::EmbeddingError;
use dotenv::dotenv;
use sqlx::{postgres::PgPool, query};
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
        CREATE TABLE IF NOT EXISTS embedding (
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

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make this test not against prod db.
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
            SELECT * FROM information_schema.tables WHERE table_name = 'embedding';
        ",
        )
        .fetch_one(&pool)
        .await;
        assert!(result.is_ok());
    }
}
