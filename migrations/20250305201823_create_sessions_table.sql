-- Add migration script here
CREATE TABLE sessions (
    id SERIAL PRIMARY KEY,
    code VARCHAR(7) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);