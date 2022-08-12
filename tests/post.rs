use actix_web::{test, web, App};
use sqlx::SqlitePool;
use web_sampler::routes::{post, Post};

#[actix_web::test]
async fn test_post() {
    let pool = SqlitePool::connect("sqlite:sampler.db")
        .await
        .expect("Failed to create connection pool");

    let new_post = Post {
        title: String::from("Lorem Ipsum"),
    };

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(post),
    )
    .await;

    let req = test::TestRequest::with_uri("/post")
        .method(actix_web::http::Method::POST)
        .set_json(new_post.clone())
        .to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let body = test::read_body(res).await;
    assert!(std::str::from_utf8(&body).unwrap().contains(&format!(
        "Your post {} has been saved under the id",
        new_post.title
    )));
}
