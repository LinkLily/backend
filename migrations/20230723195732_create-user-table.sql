-- Add migration script here

CREATE TABLE "user" (
    id uuid PRIMARY KEY,
    username text NOT NULL UNIQUE,
    name text,
    avatar_url text NOT NULL,
    created_at timestamp NOT NULL,
    email text NOT NULL UNIQUE,
    password text NOT NULL,
    salt text NOT NULL
)
