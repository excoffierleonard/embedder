mod embedding;
mod errors;

pub use embedding::{create_schema, DbPool, InputTexts, OpenAIClient};
pub use errors::EmbeddingError;
