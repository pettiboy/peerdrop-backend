-- Add migration script here
CREATE TABLE IF NOT EXISTS sessions (
    id SERIAL PRIMARY KEY,
    code VARCHAR(7) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);