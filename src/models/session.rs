use serde::Serialize;
use sqlx::FromRow;

// https://stackoverflow.com/a/78618913/14225169
#[derive(Debug, FromRow, Serialize)]
pub struct Session {
    pub id: i32,
    pub code: String,
    // idk why but allowing this to be null prevents the deserialize error
    pub created_at: Option<chrono::NaiveDateTime>
}