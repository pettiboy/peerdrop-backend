use dotenv::dotenv;
use sqlx::{Pool, Postgres, PgPool};

pub async fn get_db_pool() -> Pool<Postgres> {
    dotenv().ok();

    // construct db url
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        std::env::var("POSTGRES_USER").unwrap_err(),
        std::env::var("POSTGRES_PASSWORD").unwrap_err(),
        std::env::var("POSTGRES_HOST").unwrap_err(),
        std::env::var("POSTGRES_PORT").unwrap_err(),
        std::env::var("POSTGRES_DB").unwrap_err()
    );

    let pool = PgPool::connect(&db_url).await.unwrap();
    pool
}