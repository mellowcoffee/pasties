CREATE TABLE IF NOT EXISTS users (
    id            bigint PRIMARY KEY,
    username      text UNIQUE NOT NULL CHECK (username ~ '^[a-z0-9_-]{3,32}$'),
    bio           text NOT NULL DEFAULT '',
    avatar_url    text NOT NULL DEFAULT '',
    password_hash text NOT NULL,
    is_admin      boolean NOT NULL DEFAULT false,
    created_at    timestamptz NOT NULL DEFAULT now()
);
