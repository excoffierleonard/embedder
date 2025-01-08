use embedder::{DbPool, InputTexts, OpenAIClient};
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
    assert_eq!(embedded_texts.as_vec()[1].0, texts[1]);
    assert_eq!(embedded_texts.as_vec()[2].0, texts[2]);

    // Store the embedded texts in the database
    embedded_texts.store(&db_pool).await.unwrap();

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
