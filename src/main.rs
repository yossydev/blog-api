use actix_web::{App, HttpServer};
use likes::{get_likes, like};
mod likes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(like).service(get_likes))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
