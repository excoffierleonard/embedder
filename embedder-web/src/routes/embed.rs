//! Routes for embedding texts.

use crate::{config::LocalConfig, errors::ApiError, responses::EmbedResponse};
use actix_web::{http::header, post, web::Json};
use embedder_core::{InputTexts, OllamaClient, OpenAIClient};
use serde::Deserialize;

const OPENAI_EMBEDDING_MODELS: [&str; 3] = [
    "text-embedding-ada-002",
    "text-embedding-3-small",
    "text-embedding-3-large",
];

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

    // If the model is an openai model then use the openai api key and client, if none, then fallback to the ollama client

    // TODO: Consider not using clone here but dont fall into liftime hell
    let embeddings = match (
        OPENAI_EMBEDDING_MODELS.contains(&model.as_str()),
        openai_api_key,
    ) {
        (true, Some(api_key)) => embed_with_openai(api_key, model.clone(), texts).await?,
        (true, None) => {
            return Err(ApiError::BadRequest(
                "OpenAI API key is required".to_string(),
            ))
        }
        (false, _) => embed_with_ollama(config.ollama_api_url, model.clone(), texts).await?,
    };

    Ok(EmbedResponse { model, embeddings })
}

async fn embed_with_ollama(
    url: String,
    model: String,
    texts: Vec<String>,
) -> Result<Vec<Vec<f32>>, ApiError> {
    let client = OllamaClient::new(Some(model), Some(url));

    let embeddings = InputTexts::new(texts).embed(client).await?;

    Ok(embeddings)
}

async fn embed_with_openai(
    api_key: String,
    model: String,
    texts: Vec<String>,
) -> Result<Vec<Vec<f32>>, ApiError> {
    let client = OpenAIClient::new(api_key, Some(model));

    let embeddings = InputTexts::new(texts).embed(client).await?;

    Ok(embeddings)
}
