//! Configuration for the web server.
use dotenv::dotenv;
use std::env::var;

/// Configuration for the web server.
#[derive(Debug)]
pub struct Config {
    /// The port the server should listen on.
    pub port: u16,
}

/// Configuration for the local environment.
#[derive(Debug)]
pub struct LocalConfig {
    /// The URL of the Ollama embedding API.
    pub ollama_api_url: String,
    /// Fallback OpenAI API key.
    pub fallback_openai_api_key: Option<String>,
}

impl Config {
    /// Builds a new configuration from environment variables.
    pub fn build() -> Self {
        dotenv().ok();

        let port = var("EMBEDDER_APP_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080);

        Self { port }
    }
}

impl LocalConfig {
    /// Builds a new configuration from environment variables.
    pub fn build() -> Self {
        dotenv().ok();

        let ollama_api_url = var("OLLAMA_API_URL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or("http://localhost:11434/api/embed".to_string());

        let fallback_openai_api_key = var("OPENAI_API_KEY").ok();

        Self {
            ollama_api_url,
            fallback_openai_api_key,
        }
    }
}
