use embedder::{DbPool, EmbeddedTexts, Embedding, InputTexts, OpenAIClient};
use rand::random;

fn generate_random_text() -> String {
    format!("The sky is blue {}", random::<u32>())
}

#[tokio::test]
async fn roundtrip() {
    let openai_client = OpenAIClient::new().unwrap();
    let db_pool = DbPool::new().await.unwrap();

    let texts = vec![
        generate_random_text(),
        generate_random_text(),
        generate_random_text(),
    ];

    let first_text = texts[0].clone();

    let top_k = 3;

    // Embed the texts
    let embedded_texts = InputTexts::new(texts.clone())
        .unwrap()
        .embed(&openai_client)
        .await
        .unwrap();

    // Assertions
    assert_eq!(embedded_texts.as_vec().len(), texts.len());
    assert_eq!(embedded_texts.as_vec()[0].0, texts[0]);

    // Store the embedded texts in the database
    embedded_texts.store(&db_pool).await.unwrap();

    // Retrieve the embedded texts from the database
    let retrieved_texts = Embedding::new(embedded_texts.as_vec()[0].1.as_vec().clone())
        .unwrap()
        .fetch_similar(top_k, &db_pool)
        .await
        .unwrap();

    // Assertions
    assert_eq!(retrieved_texts.len(), top_k as usize);
    assert_eq!(retrieved_texts[0], first_text);
}
