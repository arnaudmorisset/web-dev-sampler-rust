use actix_web::{App, HttpServer};
use web_sampler::routes::{about, hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(about))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
