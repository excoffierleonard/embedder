//! Routes for embedding texts.

use crate::{config::LocalConfig, errors::ApiError, responses::EmbedResponse};
use actix_web::{http::header, post, web::Json};
use embedder_core::{InputTexts, OllamaClient};
use serde::Deserialize;

#[derive(Deserialize)]
struct EmbedRequestBody {
    model: String,
    texts: Vec<String>,
}

#[post("/embed")]
async fn embed_texts(
    req: actix_web::HttpRequest,
    Json(EmbedRequestBody { model, texts }): Json<EmbedRequestBody>,
) -> Result<EmbedResponse, ApiError> {
    let config = LocalConfig::build();

    // If header parsing return none then fallback to the fallback
    let openai_api_key = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|auth| auth.trim_start_matches("Bearer ").to_string())
        .or(config.fallback_openai_api_key);

    // Consider not using clone here
    let client = OllamaClient::new(Some(model.clone()), Some(config.ollama_api_url));

    let embeddings = InputTexts::new(texts).embed(client).await?;

    Ok(EmbedResponse { model, embeddings })
}
