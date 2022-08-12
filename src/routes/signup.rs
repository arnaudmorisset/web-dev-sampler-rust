use actix_web::{get, web, HttpResponse, Responder};

#[get("/signup/my-name-is/{name}")]
pub async fn signup(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "You're all signed up for the big convention {}!",
        name.to_string()
    ))
}
