use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use likes::{get_likes, like};
use std::sync::Mutex;
mod likes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = web::Data::new(Mutex::new(likes::init_db_connection().unwrap()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(db_conn.clone())
            .wrap(cors)
            .service(like)
            .service(get_likes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
