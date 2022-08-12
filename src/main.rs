use actix_web::{App, HttpServer};
use web_sampler::routes::{about, hello, hello_static, signup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_static)
            .service(about)
            .service(signup)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
