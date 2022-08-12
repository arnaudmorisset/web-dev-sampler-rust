use actix_web::{test, App};
use web_sampler::routes::hello;

#[actix_web::test]
async fn test_hello_world() {
    let app = test::init_service(App::new().service(hello)).await;
    let req = test::TestRequest::default().to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let body = test::read_body(res).await;
    assert_eq!(body, "Hello, World!");
}
