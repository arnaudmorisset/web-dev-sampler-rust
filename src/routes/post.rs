use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
}

#[post("/post")]
pub async fn post(post: web::Json<Post>) -> impl Responder {
    HttpResponse::Ok().body(format!("Your post will be named {}", post.title))
}
