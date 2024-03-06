use actix_web::{get, post, web, HttpResponse, Responder};
use rusqlite::{params, Connection, Result};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
struct IncrementRequest {
    increment: i32,
}

#[derive(Serialize)]
struct LikeResponse {
    slug: String,
    likes: i32,
}

#[post("/like/{slug}")]
pub async fn like(
    slug: web::Path<String>,
    increment: web::Json<IncrementRequest>,
) -> impl Responder {
    let slug_value = slug.into_inner();
    let increment_value = increment.into_inner().increment;

    match update_or_insert_like(&slug_value, increment_value) {
        Ok(total) => HttpResponse::Ok().json(LikeResponse {
            slug: slug_value,
            likes: total,
        }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

fn update_or_insert_like(slug: &str, increment: i32) -> Result<i32> {
    let conn = Connection::open("likes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS likes (
            id INTEGER PRIMARY KEY,
            slug TEXT NOT NULL UNIQUE,
            count INTEGER DEFAULT 0
        )",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT count FROM likes WHERE slug = ?")?;
    let mut count_iter = stmt.query_map(params![slug], |row| row.get(0))?;
    let count: i32 = if let Some(result) = count_iter.next() {
        result?
    } else {
        0
    };

    let new_count = count + increment;

    conn.execute(
        "INSERT INTO likes (slug, count) VALUES (?1, ?2)
         ON CONFLICT(slug) DO UPDATE SET count = ?2",
        params![slug, new_count],
    )?;

    Ok(new_count)
}

#[get("/like/{slug}")]
pub async fn get_likes(slug: web::Path<String>) -> impl Responder {
    match fetch_likes(&slug) {
        Ok(likes) => HttpResponse::Ok().json(likes),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

fn fetch_likes(slug: &str) -> Result<LikeResponse> {
    let conn = Connection::open("likes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS likes (
            id INTEGER PRIMARY KEY,
            slug TEXT NOT NULL UNIQUE,
            count INTEGER DEFAULT 0
        )",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT count FROM likes WHERE slug = ?")?;
    let count: i32 = stmt.query_row(params![slug], |row| row.get(0)).unwrap_or(0);

    Ok(LikeResponse {
        slug: slug.to_string(),
        likes: count,
    })
}
