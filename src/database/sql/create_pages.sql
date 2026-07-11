CREATE TABLE IF NOT EXISTS pages (
    id         bigint PRIMARY KEY,
    slug       text UNIQUE NOT NULL CHECK (slug ~ '^[a-z0-9_-]{3,64}$'),
    owner_id   bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    html       text NOT NULL,
    css        text NOT NULL,
    views      bigint NOT NULL DEFAULT 0,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS pages_owner_id_idx ON pages (owner_id);
