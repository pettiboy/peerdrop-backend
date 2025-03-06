use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::utils::generate_code;
use crate::db::session;

#[post("/sessions")]
async fn create_session(pool: web::Data<PgPool>) -> impl Responder {
    let code = generate_code::generate_code(7);

    match session::create_session(&pool, &code).await {
        Ok(session) => HttpResponse::Ok().json(session),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "message": "Something went wrong"
        }))
    }

}

#[get("/sessions/{code}")]
async fn get_session(pool: web::Data<PgPool>, code: web::Path<String>) -> impl Responder {
    match session::get_session(&pool, &code).await {
        Ok(session) => HttpResponse::Ok().json(session),
        Err(_) => HttpResponse::NotFound().json(json!({
            "message": "session id not found"
        }))
    }
}