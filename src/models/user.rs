use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,                  // primary key
    pub code: String,             // for qr code
    pub ecdh_public_key: String,  // ECDH public key | 64 char hex
    pub eddsa_public_key: String, // EdDSA public key | 64 char hex

    pub created_at: Option<DateTime<Utc>>,
}
