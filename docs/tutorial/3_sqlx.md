# How to create new table in DB

In this example I will create new User table

## create migration

```sh
sqlx migrate add create_users_table
```

## add commands to the generated `.sql` file

```sql
-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    code VARCHAR(255) UNIQUE NOT NULL,
    ecdh_public_key CHAR(64) UNIQUE NOT NULL,
    eddsa_public_key CHAR(64) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

## apply migrations to db

```sh
sqlx migrate run
```

## define model

models/mod.rs

```rs
pub mod user;
```

models/user.rs

```rs
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    pub id: i32,                  // primary key
    pub code: String,             // for qr code
    pub ecdh_public_key: String,  // ECDH public key | 64 char hex
    pub eddsa_public_key: String, // EdDSA public key | 64 char hex

    pub created_at: Option<DateTime<Utc>>, // timestamp with timezone
}
```
