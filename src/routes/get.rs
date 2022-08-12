use actix_web::{get, http, web, HttpResponse};
use serde::Serialize;
use sqlx::SqlitePool;

use crate::routes::Post;

#[derive(Serialize)]
struct Error {
    message: String,
}

#[get("/post/{uuid}")]
pub async fn get(pool: web::Data<SqlitePool>, uuid: web::Path<String>) -> HttpResponse {
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection from pool");

    let post_uuid = uuid.to_string();
    match sqlx::query!(
        r#"
            SELECT *
            FROM posts
            WHERE id = $1
        "#,
        post_uuid
    )
    .fetch_one(&mut conn)
    .await
    {
        Ok(post) => {
            let serialized = serde_json::to_string(&Post {
                id: post.id,
                title: post.title,
            })
            .unwrap();
            HttpResponse::Ok()
                .content_type(http::header::ContentType::json())
                .body(serialized)
        }
        Err(e) => {
            let serialize = serde_json::to_string(&Error {
                message: e.to_string(),
            })
            .unwrap();
            HttpResponse::InternalServerError()
                .content_type(http::header::ContentType::json())
                .body(serialize)
        }
    }
}
