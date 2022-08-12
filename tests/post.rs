use actix_web::{test, App};
use web_sampler::routes::{post, Post};

#[actix_web::test]
async fn test_post() {
    let new_post = Post {
        title: String::from("Lorem Ipsum"),
    };

    let app = test::init_service(App::new().service(post)).await;
    let req = test::TestRequest::with_uri("/post")
        .method(actix_web::http::Method::POST)
        .set_json(new_post.clone())
        .to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let body = test::read_body(res).await;
    assert_eq!(body, format!("Your post will be named {}", new_post.title));
}
