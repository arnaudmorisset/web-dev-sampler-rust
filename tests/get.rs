use actix_web::{test, web, App};
use sqlx::SqlitePool;
use uuid::Uuid;
use web_sampler::routes::{get, Post};

async fn setup_db(pool: &SqlitePool) -> String {
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection from pool");

    let uuid = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
            INSERT INTO posts (id, title)
            VALUES ($1, $2)
        "#,
        uuid,
        "Test Post"
    )
    .execute(&mut conn)
    .await
    .expect("Failed to setup database");

    return uuid;
}

#[actix_web::test]
async fn test_get() {
    let pool = SqlitePool::connect("sqlite:sampler.db")
        .await
        .expect("Failed to create connection pool");

    let uuid = setup_db(&pool).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get),
    )
    .await;

    let req = test::TestRequest::with_uri(&format!("/post/{}", uuid)).to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());

    let body = test::read_body(res).await;
    let post: Post = serde_json::from_str(std::str::from_utf8(&body).unwrap()).unwrap();

    assert_eq!(post.id, uuid);
    assert_eq!(post.title, "Test Post");
}
