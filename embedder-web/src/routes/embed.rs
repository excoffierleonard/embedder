use crate::{config::LocalConfig, errors::ApiError, responses::EmbedResponse};
use actix_web::{http::header, post, web::Json};
use embedder_core::{InputTexts, OllamaClient, OpenAIClient, DEFAULT_OLLAMA_EMBEDDING_MODEL};
use serde::Deserialize;

const OPENAI_EMBEDDING_MODELS: [&str; 2] = ["text-embedding-3-small", "text-embedding-3-large"];

#[derive(Deserialize)]
struct EmbedRequestBody {
    /// Make the model optional. If not provided, we default to Ollama.
    #[serde(default)]
    model: Option<String>,
    texts: Vec<String>,
}

#[post("/embed")]
async fn embed_texts(
    req: actix_web::HttpRequest,
    Json(EmbedRequestBody { model, texts }): Json<EmbedRequestBody>,
) -> Result<EmbedResponse, ApiError> {
    let config = LocalConfig::build();

    // Try to extract the OpenAI API key from the Authorization header, falling back if needed.
    let openai_api_key = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|auth| auth.trim_start_matches("Bearer ").to_string())
        .or(config.fallback_openai_api_key);

    let model_for_response = model.clone();
    let embeddings = match (model, openai_api_key) {
        // 1. No model provided: default to Ollama.
        (None, _) => embed_with_ollama(config.ollama_api_url, None, texts).await?,

        // 2. An OpenAI model is specified and an API key is provided: use OpenAI.
        (Some(ref m), Some(ref api_key)) if OPENAI_EMBEDDING_MODELS.contains(&m.as_str()) => {
            // Clone m and api_key as needed.
            embed_with_openai(api_key.clone(), m.clone(), texts).await?
        }

        // 3. An OpenAI model is specified but no API key is provided: error.
        (Some(m), None) if OPENAI_EMBEDDING_MODELS.contains(&m.as_str()) => {
            return Err(ApiError::BadRequest(
                "OpenAI API key is required".to_string(),
            ))
        }

        // 4. A model is specified but it is not an OpenAI model: use Ollama.
        (Some(m), _) => embed_with_ollama(config.ollama_api_url, Some(m), texts).await?,
    };
    // If no model was provided, use the default Ollama model.
    let response_model =
        model_for_response.unwrap_or_else(|| DEFAULT_OLLAMA_EMBEDDING_MODEL.to_string());
    Ok(EmbedResponse {
        model: response_model,
        embeddings,
    })
}

async fn embed_with_ollama(
    url: String,
    model: Option<String>,
    texts: Vec<String>,
) -> Result<Vec<Vec<f32>>, ApiError> {
    let client = OllamaClient::new(model, Some(url));
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
