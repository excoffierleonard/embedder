mod embedding;
mod errors;

pub use embedding::{DbPool, EmbeddedTexts, Embedding, InputTexts, OpenAIClient};
pub use errors::EmbeddingError;
