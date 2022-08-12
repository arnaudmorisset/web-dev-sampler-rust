use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use web_sampler::routes::{about, hello, hello_static, parse, post, signup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("sqlite:sampler.db")
        .await
        .expect("Failed to create connection pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
