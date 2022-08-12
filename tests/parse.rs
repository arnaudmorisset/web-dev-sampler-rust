use actix_web::{http::header::ContentType, test, App};
use web_sampler::routes::{parse, Sample};

#[actix_web::test]
async fn test_parse() {
    let app = test::init_service(App::new().service(parse)).await;
    let req = test::TestRequest::with_uri("/parse").to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let content_type = res.headers().get("Content-Type").unwrap().to_str().unwrap();
    assert_eq!(content_type, ContentType::json().to_string());

    let sample = Sample {
        id: 1,
        name: String::from("Arnaud"),
    };

    let expected_body = serde_json::to_string(&sample).unwrap();

    let body = test::read_body(res).await;
    assert_eq!(body, expected_body);
}
