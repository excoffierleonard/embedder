use actix_web::{test, App};
use embedder_web::routes::embed_texts;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct EmbedRequestBody {
    model: String,
    texts: Vec<String>,
}

#[derive(Deserialize)]
struct EmbedResponseBody {
    model: String,
    embeddings: Vec<Vec<f32>>,
}

#[actix_web::test]
async fn request_embed_ollama_success() {
    // Setup
    let app = test::init_service(App::new().service(embed_texts)).await;

    // Create body
    let model = "nomic-embed-text".to_string();
    let texts = vec!["Hello, world!".to_string(), "Goodbye, world!".to_string()];

    let body = EmbedRequestBody {
        model: model.clone(),
        texts: texts.clone(),
    };

    // Create request
    let req = test::TestRequest::post()
        .uri("/embed")
        .set_json(&body)
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    let status = resp.status();
    assert!(status.is_success());

    let body: EmbedResponseBody = test::read_body_json(resp).await;
    assert_eq!(body.model, model);
    assert_eq!(body.embeddings.len(), texts.len());
}

#[actix_web::test]
async fn request_parse_openai_custom_success() {}

#[actix_web::test]
async fn request_parse_openai_default_success() {}
