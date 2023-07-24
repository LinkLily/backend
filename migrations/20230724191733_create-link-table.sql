-- Add migration script here

CREATE TABLE link (
    id uuid PRIMARY KEY NOT NULL,
    for_username text REFERENCES "user" (username) NOT NULL,
    "order" int NOT NULL,
    label text NOT NULL,
    link text NOT NULL,
    is_nsfw bool NOT NULL
)
