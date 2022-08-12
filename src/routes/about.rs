use actix_web::{get, http::header::ContentType, HttpResponse, Responder};

#[get("/about")]
pub async fn about() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<h1>This is the about page</h1>")
}
