-- Add migration script here

CREATE TABLE IF NOT EXISTS link (
    id uuid PRIMARY KEY,
    for_username text REFERENCES "user" (username) NOT NULL,
    "order" int NOT NULL,
    label text NOT NULL,
    link text NOT NULL,
    is_nsfw bool NOT NULL
)
