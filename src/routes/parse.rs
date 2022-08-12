use actix_web::{get, http::header::ContentType, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct Sample {
    pub id: u32,
    pub name: String,
}

#[get("/parse")]
pub async fn parse() -> impl Responder {
    let sample = Sample {
        id: 1,
        name: String::from("Arnaud"),
    };

    let serialized = serde_json::to_string(&sample).unwrap();

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serialized)
}
