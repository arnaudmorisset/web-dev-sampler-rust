use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
}

#[post("/post")]
pub async fn post(pool: web::Data<SqlitePool>, post: web::Json<Post>) -> HttpResponse {
    let mut conn = pool
        .acquire()
        .await
        .expect("Failed to acquire connection from pool");

    let uuid = Uuid::new_v4().to_string();

    match sqlx::query!(
        r#"
            INSERT INTO posts (id, title)
            VALUES ($1, $2)
        "#,
        uuid,
        post.title
    )
    .execute(&mut conn)
    .await
    {
        Ok(_) => HttpResponse::Ok().body(format!(
            "Your post {} has been saved under the id {}",
            post.title, uuid
        )),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to save your post: {:?}", e))
        }
    }
}
