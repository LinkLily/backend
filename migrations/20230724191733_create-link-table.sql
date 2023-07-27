-- Add migration script here

CREATE TABLE IF NOT EXISTS link (
    id uuid PRIMARY KEY,
    user_id uuid REFERENCES "user" (id) NOT NULL,
    "order" int NOT NULL,
    label text NOT NULL,
    link text NOT NULL,
    is_nsfw bool NOT NULL
)
