use actix_web::{test, App};
use web_sampler::routes::signup;

#[actix_web::test]
async fn test_signup() {
    let app = test::init_service(App::new().service(signup)).await;
    let req = test::TestRequest::with_uri("/signup/my-name-is/Arnaud").to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let body = test::read_body(res).await;
    assert_eq!(body, "You're all signed up for the big convention Arnaud!");
}
