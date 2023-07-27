-- Add migration script here

CREATE TABLE "user" (
    id uuid PRIMARY KEY,
    username text NOT NULL UNIQUE,
    "name" text,
    email text NOT NULL UNIQUE,
    "password" text NOT NULL,
    salt text NOT NULL,
    avatar_url text NOT NULL,
    created_at timestamp NOT NULL,
    "role" text NOT NULL
)
