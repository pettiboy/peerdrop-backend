use crate::{db::{session, user}, utils::generate_code};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    ecdh_public_key: String,
    eddsa_public_key: String,
}

#[post("/users")]
async fn create_user(pool: web::Data<PgPool>, req: web::Json<CreateUserRequest>) -> impl Responder {
    let code = generate_code::generate_code(7);

    println!("{:?}", req.ecdh_public_key);

    match user::create_user(&pool, &code, &req.ecdh_public_key, &req.eddsa_public_key).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "message": "Something went wrong"
        })),
    }}

#[get("/users/{code}")]
async fn get_user(pool: web::Data<PgPool>, code: web::Path<String>) -> impl Responder {
    match session::get_session(&pool, &code).await {
        Ok(session) => HttpResponse::Ok().json(session),
        Err(_) => HttpResponse::NotFound().json(json!({
            "message": "session id not found"
        })),
    }
}
