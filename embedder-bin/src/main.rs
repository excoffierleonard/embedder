use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use embedder_web::{routes::embed_texts, Config};
use env_logger::{init_from_env, Env};
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    init_from_env(Env::default().default_filter_or("info"));

    let config = Config::build();

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(embed_texts)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
