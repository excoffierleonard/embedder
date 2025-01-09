use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

/// Response type for greeting
#[derive(Serialize)]
pub struct EmbedResponse {
    /// The embedding model name
    pub model: String,
    /// The embeddings of the texts
    pub embeddings: Vec<Vec<f32>>,
}

impl Responder for EmbedResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
