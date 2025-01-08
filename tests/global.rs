use embedder::{create_schema, DbPool, InputTexts, OpenAIClient};
use rand::random;
use sqlx::query;

fn generate_random_text() -> String {
    format!("The sky is blue {}", random::<u32>())
}

#[tokio::test]
async fn create_schema_success() {
    let pool = DbPool::new().await.unwrap();
    create_schema(&pool).await.unwrap();

    // Check that the vector extension was created
    let result = query(
        "
            SELECT * 
            FROM pg_extension 
            WHERE extname = 'vector';
        ",
    )
    .fetch_one(pool.inner())
    .await;
    assert!(result.is_ok());

    // Check that the embedding table was created
    let result = query(
        "
            SELECT * 
            FROM information_schema.tables 
            WHERE table_name = 'embeddings';
        ",
    )
    .fetch_one(pool.inner())
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn roundtrip() {
    // Input setup
    let openai_client = OpenAIClient::new().unwrap();
    let db_pool = DbPool::new().await.unwrap();
    create_schema(&db_pool).await.unwrap();

    let texts = vec![
        generate_random_text(),
        generate_random_text(),
        generate_random_text(),
    ];

    // Embed the texts
    let embedded_texts = InputTexts::new(texts.clone())
        .unwrap()
        .embed(&openai_client)
        .await
        .unwrap();

    // Assertions
    assert_eq!(embedded_texts.as_vec().len(), texts.len());
    assert_eq!(embedded_texts.as_vec()[0].0, texts[0]);
    assert_eq!(embedded_texts.as_vec()[1].0, texts[1]);
    assert_eq!(embedded_texts.as_vec()[2].0, texts[2]);

    // Store the embedded texts in the database
    embedded_texts.store(&db_pool).await.unwrap();

    // Input setup
    let top_k = 3;

    // Retrieve the embedded texts from the database
    let retrieved_texts = InputTexts::new(texts.clone())
        .unwrap()
        .embed(&openai_client)
        .await
        .unwrap()
        .fetch_similar(top_k, &db_pool)
        .await
        .unwrap();

    // Assertions
    assert_eq!(retrieved_texts[0].len(), top_k as usize);
    assert_eq!(retrieved_texts[0][0], texts[0]);

    assert_eq!(retrieved_texts[1].len(), top_k as usize);
    assert_eq!(retrieved_texts[1][0], texts[1]);

    assert_eq!(retrieved_texts[2].len(), top_k as usize);
    assert_eq!(retrieved_texts[2][0], texts[2]);
}
