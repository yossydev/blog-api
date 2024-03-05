use actix_cors::Cors;
use actix_web::{App, HttpServer};
use likes::{get_likes, like};
mod likes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);
        App::new().wrap(cors).service(like).service(get_likes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
