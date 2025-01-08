use embedder::{DbPool, InputTexts, OpenAIClient};
use serde_json::Value;

#[tokio::test]
async fn input() {
    let openai_client = OpenAIClient::new().unwrap();
    let db_pool = DbPool::new().await.unwrap();

    // Create the vector of texts from the json dataset
    let json_str = include_str!("ressources/countries.json");
    let data: Vec<Value> = serde_json::from_str(json_str).unwrap();

    let texts: Vec<String> = data
        .iter()
        .filter_map(|country| {
            country["introduction_background"]
                .as_str()
                .map(String::from)
        })
        .collect();

    InputTexts::new(texts)
        .unwrap()
        .embed(&openai_client)
        .await
        .unwrap()
        .store(&db_pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn output() {
    let openai_client = OpenAIClient::new().unwrap();
    let db_pool = DbPool::new().await.unwrap();

    let text = vec!["A country with great food, a rich history, and beautiful women.".to_string()];

    let top_k = 3;

    let retrieved_texts = InputTexts::new(text.clone())
        .unwrap()
        .embed(&openai_client)
        .await
        .unwrap()
        .fetch_similar(top_k, &db_pool)
        .await
        .unwrap();

    assert_eq!(retrieved_texts.len(), text.len());
    assert_eq!(retrieved_texts[0].len(), top_k as usize);

    for (i, retrieved) in retrieved_texts[0].iter().enumerate() {
        println!("Result {}: {}", i + 1, retrieved);
    }
}
