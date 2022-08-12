use actix_web::{http::header::ContentType, test, App};
use web_sampler::routes::hello;

#[actix_web::test]
async fn test_hello_world() {
    let app = test::init_service(App::new().service(hello)).await;
    let req = test::TestRequest::default().to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let content_type = res.headers().get("Content-Type").unwrap().to_str().unwrap();
    assert_eq!(content_type, ContentType::html().to_string());

    let body = test::read_body(res).await;
    assert_eq!(body, "<h1>Hello, World!</h1>");
}
