//! Routes for embedding texts.

use crate::{errors::ApiError, responses::EmbedResponse};
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
    let client = OllamaClient::default();

    let embeddings = InputTexts::new(texts).embed(client).await?;

    Ok(EmbedResponse {
        model: model,
        embeddings: embeddings,
    })
}
