-- Add migration script here

CREATE TABLE api_key (
    id uuid PRIMARY KEY NOT NULL,
    hashed_key text NOT NULL UNIQUE,
    permission_level int NOT NULL
)
