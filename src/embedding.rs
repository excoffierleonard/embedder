use crate::errors::EmbeddingError;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, query, QueryBuilder, Row};
use std::env::var;

const EMBEDDING_DIMENSION: usize = 3072;
const OPENAI_API_URL: &str = "https://api.openai.com/v1/embeddings";
const EMBEDDING_MODEL: &str = "text-embedding-3-large";

/// OpenAI API client
#[derive(Debug)]
pub struct OpenAIClient {
    api_key: String,
}

/// Constructor implementation for OpenAIClient
impl OpenAIClient {
    /// Creates a new OpenAIClient instance
    pub fn new() -> Result<Self, EmbeddingError> {
        dotenv().ok();
        let api_key = var("OPENAI_API_KEY")?;

        Ok(Self { api_key })
    }
}

/// Database connection pool
#[derive(Debug)]
pub struct DbPool(PgPool);

/// Constructor implementation for DbPool
impl DbPool {
    /// Creates a new DbPool instance
    pub async fn new() -> Result<Self, EmbeddingError> {
        dotenv().ok();
        let database_url = var("DATABASE_URL")?;
        let pool = PgPool::connect(&database_url).await?;
        Ok(Self(pool))
    }
}

/// Input texts to be embedded
#[derive(Debug)]
pub struct InputTexts(Vec<String>);

/// Constructor implementation for InputTexts
impl InputTexts {
    /// Creates a new InputTexts instance
    pub fn new(texts: Vec<String>) -> Result<Self, EmbeddingError> {
        if texts.is_empty() {
            return Err(EmbeddingError::EmptyInput);
        }

        // Additional validation could be added here
        for text in &texts {
            if text.trim().is_empty() {
                return Err(EmbeddingError::EmptyString);
            }
        }

        Ok(Self(texts))
    }

    /// Returns a reference to the inner vector of strings
    pub fn as_vec(&self) -> &Vec<String> {
        &self.0
    }

    /// Embeds the input texts using OpenAI's API
    pub async fn embed(
        self,
        openai_client: &OpenAIClient,
    ) -> Result<EmbeddedTexts, EmbeddingError> {
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
            input: self.0,
            model: EMBEDDING_MODEL.to_string(),
        };

        let client = Client::new();
        let response: OpenAIResponse = client
            .post(OPENAI_API_URL)
            .bearer_auth(&openai_client.api_key)
            .json(&request)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        // Get texts back from the request after it's used
        let texts = InputTexts::new(request.input)?;

        // Convert response into embeddings
        let embeddings = response
            .data
            .into_iter()
            .map(|e| Embedding::new(e.embedding))
            .collect::<Result<Vec<_>, _>>()?;

        EmbeddedTexts::new(texts, embeddings)
    }
}

/// Embedding of a text
#[derive(Debug)]
pub struct Embedding(Vec<f32>);

/// Constructor implementation for Embedding
impl Embedding {
    /// Creates a new Embedding instance
    fn new(embedding: Vec<f32>) -> Result<Self, EmbeddingError> {
        if embedding.len() != EMBEDDING_DIMENSION {
            return Err(EmbeddingError::InvalidDimension {
                expected: EMBEDDING_DIMENSION,
                got: embedding.len(),
            });
        }

        // Additional validation could be added here
        if embedding.iter().any(|&x| x.is_nan() || x.is_infinite()) {
            return Err(EmbeddingError::InvalidValues);
        }

        Ok(Self(embedding))
    }

    /// Returns a reference to the inner vector of f32 values
    pub fn as_vec(&self) -> &Vec<f32> {
        &self.0
    }

    /// Returns the most similar texts to the given embedding
    async fn fetch_similar(
        &self,
        top_k: i32,
        pool: &DbPool,
    ) -> Result<Vec<String>, EmbeddingError> {
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
        .bind(self.as_vec())
        .bind(top_k)
        .fetch_all(&pool.0)
        .await?;

        Ok(result.iter().map(|row| row.get("text")).collect())
    }
}

/// Texts with their embeddings
#[derive(Debug)]
pub struct EmbeddedTexts(Vec<(String, Embedding)>);

/// Constructor implementation for EmbeddedTexts
impl EmbeddedTexts {
    /// Creates a new EmbeddedTexts instance
    fn new(texts: InputTexts, embeddings: Vec<Embedding>) -> Result<Self, EmbeddingError> {
        if texts.as_vec().len() != embeddings.len() {
            return Err(EmbeddingError::MismatchedLength {
                texts: texts.as_vec().len(),
                embeddings: embeddings.len(),
            });
        }

        Ok(Self(
            texts
                .as_vec()
                .into_iter()
                .map(|s| s.to_string())
                .zip(embeddings)
                .collect(),
        ))
    }

    /// Returns a reference to the inner vector of text-embedding pairs
    pub fn as_vec(&self) -> &Vec<(String, Embedding)> {
        &self.0
    }

    /// Stores the embedded texts in the database
    pub async fn store(&self, pool: &DbPool) -> Result<(), EmbeddingError> {
        let mut query_builder = QueryBuilder::new("INSERT INTO embeddings (text, embedding) ");

        query_builder.push_values(&self.0, |mut b, pair| {
            let (text, embedding) = pair;
            b.push_bind(text).push_bind(embedding.as_vec());
        });

        query_builder.build().execute(&pool.0).await?;

        Ok(())
    }

    /// Fetches the most similar texts to the given embeddings
    pub async fn fetch_similar(
        &self,
        top_k: i32,
        pool: &DbPool,
    ) -> Result<Vec<Vec<String>>, EmbeddingError> {
        let mut result = Vec::new();

        for (_, embedding) in self.as_vec().iter() {
            let similar_texts = embedding.fetch_similar(top_k, pool).await?;
            result.push(similar_texts);
        }

        Ok(result)
    }
}

/// Initializes the database by creating the necessary tables and extensions
pub async fn create_schema(pool: &DbPool) -> Result<(), EmbeddingError> {
    // Create the vector extension
    query("CREATE EXTENSION IF NOT EXISTS vector;")
        .execute(&pool.0)
        .await?;

    // Create the embeddings table
    query(&format!(
        "
        CREATE TABLE IF NOT EXISTS embeddings (
            id UUID DEFAULT gen_random_uuid(),
            text TEXT NOT NULL,
            embedding vector({}),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (id)
        );
        ",
        EMBEDDING_DIMENSION
    ))
    .execute(&pool.0)
    .await?;

    Ok(())
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
    async fn input_texts_embed_success() {
        let openai_client = OpenAIClient::new().unwrap();
        let texts = vec![generate_random_text(), generate_random_text()];
        let result = InputTexts::new(texts.clone())
            .unwrap()
            .embed(&openai_client)
            .await
            .unwrap();

        assert_eq!(result.as_vec().len(), texts.len());
        assert_eq!(result.as_vec()[0].1.as_vec().len(), EMBEDDING_DIMENSION);
        assert_eq!(result.as_vec()[1].1.as_vec().len(), EMBEDDING_DIMENSION);
        assert_eq!(result.as_vec()[0].0, texts[0]);
        assert_eq!(result.as_vec()[1].0, texts[1]);
    }

    #[tokio::test]
    async fn embedding_store_success() {
        let pool = DbPool::new().await.unwrap();
        create_schema(&pool).await.unwrap();
        let text = generate_random_text();
        let embedding = generate_random_embedding();

        EmbeddedTexts::new(
            InputTexts::new(vec![text.clone()]).unwrap(),
            vec![Embedding::new(embedding.clone()).unwrap()],
        )
        .unwrap()
        .store(&pool)
        .await
        .unwrap();

        // Check that the row was inserted
        let result = query(
            "
                SELECT * FROM embeddings WHERE text = $1;
            ",
        )
        .bind(text)
        .fetch_one(&pool.0)
        .await;

        assert!(result.is_ok());
    }

    // TODO: Make these tests not against prod db. And also the other test not depend on that one since they may run in parralel.

    #[tokio::test]
    async fn create_schema_success() {
        let pool = DbPool::new().await.unwrap();
        create_schema(&pool).await.unwrap();

        // Check that the vector extension was created
        let result = query(
            "
                SELECT * FROM pg_extension WHERE extname = 'vector';
            ",
        )
        .fetch_one(&pool.0)
        .await;
        assert!(result.is_ok());

        // Check that the embedding table was created
        let result = query(
            "
                SELECT * FROM information_schema.tables WHERE table_name = 'embeddings';
            ",
        )
        .fetch_one(&pool.0)
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_similar_success() {
        let pool = DbPool::new().await.unwrap();
        create_schema(&pool).await.unwrap();
        let text = generate_random_text();
        let embedding = generate_random_embedding();

        EmbeddedTexts::new(
            InputTexts::new(vec![text.clone()]).unwrap(),
            vec![Embedding::new(embedding.clone()).unwrap()],
        )
        .unwrap()
        .store(&pool)
        .await
        .unwrap();

        let top_k = 5;

        let similar_texts = Embedding::new(embedding)
            .unwrap()
            .fetch_similar(top_k.clone(), &pool)
            .await
            .unwrap();

        assert_eq!(similar_texts.len(), top_k as usize);
        assert_eq!(similar_texts[0], text);
    }
}
