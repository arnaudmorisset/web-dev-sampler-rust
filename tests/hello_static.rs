use actix_web::{http::header::ContentType, test, App};
use std::fs;
use web_sampler::routes::hello_static;

#[actix_web::test]
async fn test_hello_static() {
    let app = test::init_service(App::new().service(hello_static)).await;
    let req = test::TestRequest::with_uri("/hello").to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let content_type = res.headers().get("Content-Type").unwrap().to_str().unwrap();
    assert_eq!(content_type, ContentType::html().to_string());

    let body = test::read_body(res).await;
    assert_eq!(body, response_body());
}

fn response_body() -> String {
    fs::read_to_string("./public/static.html").expect("Failed to fetch static file content")
}
