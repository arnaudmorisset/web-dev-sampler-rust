use actix_web::{http::header::ContentType, test, App};
use web_sampler::routes::about;

#[actix_web::test]
async fn test_about_page() {
    let app = test::init_service(App::new().service(about)).await;
    let req = test::TestRequest::with_uri("/about").to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let content_type = res.headers().get("Content-Type").unwrap().to_str().unwrap();
    assert_eq!(content_type, ContentType::html().to_string());

    let body = test::read_body(res).await;
    assert_eq!(body, "<h1>This is the about page</h1>");
}
