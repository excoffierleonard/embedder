//! Routes for embedding texts.

use crate::{config::LocalConfig, errors::ApiError, responses::EmbedResponse};
use actix_web::{post, web::Json};
use embedder_core::{InputTexts, OllamaClient};
use serde::Deserialize;

#[derive(Deserialize)]
struct EmbedRequestBody {
    model: String,
    texts: Vec<String>,
}

#[post("/embed")]
async fn embed_texts(
    Json(EmbedRequestBody { model, texts }): Json<EmbedRequestBody>,
) -> Result<EmbedResponse, ApiError> {
    let config = LocalConfig::build();

    // Consider not using clone here
    let client = OllamaClient::new(Some(model.clone()), Some(config.ollama_api_url));

    let embeddings = InputTexts::new(texts).embed(client).await?;

    Ok(EmbedResponse { model, embeddings })
}
