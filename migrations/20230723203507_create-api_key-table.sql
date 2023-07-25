-- Add migration script here

CREATE TABLE IF NOT EXISTS api_key (
    id uuid PRIMARY KEY,
    hashed_key text NOT NULL UNIQUE,
    permission_level int NOT NULL
)
