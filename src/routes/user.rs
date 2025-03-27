use crate::{db::user, utils::generate_code};
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

    match user::create_user(&pool, &code, &req.ecdh_public_key, &req.eddsa_public_key).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "message": err.to_string(),
        })),
    }
}

#[get("/users/{code}")]
async fn get_user(pool: web::Data<PgPool>, code: web::Path<String>) -> impl Responder {
    match user::get_user(&pool, &code).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().json(json!({
            "message": "user code not found"
        })),
    }
}
