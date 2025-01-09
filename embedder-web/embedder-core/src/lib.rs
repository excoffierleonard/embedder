//! Core embedder library.

mod embedders;
mod errors;

pub use embedders::{InputTexts, OllamaClient, OpenAIClient};
pub use errors::EmbedderError;
