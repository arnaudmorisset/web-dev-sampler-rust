use actix_files::NamedFile;
use actix_web::{get, Result};

#[get("/hello")]
pub async fn hello_static() -> Result<NamedFile> {
    Ok(NamedFile::open("./public/static.html")?)
}
