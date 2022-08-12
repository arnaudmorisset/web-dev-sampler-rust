use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<h1>Hello, World!</h1>")
}
