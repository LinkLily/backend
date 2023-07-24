-- Add migration script here

CREATE TABLE "user" (
    username text PRIMARY KEY NOT NULL,
    name text,
    avatar_url text NOT NULL,
    created_at text NOT NULL,
    email text NOT NULL UNIQUE,
    password text NOT NULL,
    salt text NOT NULL
)
