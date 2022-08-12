use actix_web::{get, web, HttpResponse, Responder};
use sanitize_html::rules::predefined::DEFAULT;
use sanitize_html::sanitize_str;

#[get("/signup/my-name-is/{name}")]
pub async fn signup(name: web::Path<String>) -> impl Responder {
    let sanitized_name = sanitize_str(&DEFAULT, name.as_str()).unwrap();

    HttpResponse::Ok().body(format!(
        "You're all signed up for the big convention {}!",
        sanitized_name
    ))
}
