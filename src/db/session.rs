use sqlx::PgPool;
use crate::models::session::Session;

pub async fn create_session(pool: &PgPool, code: &str) -> Result<Session, sqlx::Error> {
    sqlx::query_as!(
        Session,
        r#"
            INSERT INTO sessions (code)
            VALUES ($1)
            RETURNING id, code, created_at
        "#,
        code
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
