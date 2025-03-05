use dotenv::dotenv;
use sqlx::{Pool, Postgres, PgPool};

pub async fn get_db_pool() -> Pool<Postgres> {
    dotenv().ok();

    // construct db url
    let db_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&db_url).await.unwrap();
    pool
}