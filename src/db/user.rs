use crate::models::{session::Session, user::User};
use sqlx::PgPool;

// todo handle name here?
pub async fn create_user(
    pool: &PgPool,
    code: &str,
    ecdh_public_key: &str,
    eddsa_public_key: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            INSERT INTO users (code, ecdh_public_key, eddsa_public_key)
            VALUES ($1, $2, $3)
            RETURNING id, code, name, ecdh_public_key, eddsa_public_key, created_at
        "#,
        code,
        ecdh_public_key,
        eddsa_public_key
    )
    .fetch_one(pool)
    .await
}

pub async fn get_session(pool: &PgPool, code: &str) -> Result<Session, sqlx::Error> {
    sqlx::query_as!(
        Session,
        r#"
            SELECT * 
            FROM sessions 
            WHERE code = $1
        "#,
        code
    )
    .fetch_one(pool)
    .await
}
