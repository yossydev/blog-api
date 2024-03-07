use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
use likes::{get_likes, like};
mod likes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_origin("http://localhost:8788")
                    .allowed_origin("yossydev-blog.pages.dev")
                    .allowed_origin("yossy.dev")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .service(like)
            .service(get_likes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
