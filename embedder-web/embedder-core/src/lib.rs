//! Core embedder library.

mod embedders;
mod errors;

pub use embedders::{InputTexts, OllamaClient, OpenAIClient, DEFAULT_OLLAMA_EMBEDDING_MODEL};
pub use errors::EmbedderError;
