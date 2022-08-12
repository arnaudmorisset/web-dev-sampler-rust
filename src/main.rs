use actix_web::{App, HttpServer};
use web_sampler::routes::{about, hello, hello_static, parse, post, signup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_static)
            .service(about)
            .service(signup)
            .service(parse)
            .service(post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
