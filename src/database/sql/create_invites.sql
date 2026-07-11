CREATE TABLE IF NOT EXISTS invites (
    code       text PRIMARY KEY,
    created_by bigint NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    used_by    bigint          REFERENCES users(id) ON DELETE CASCADE,
    created_at timestamptz NOT NULL DEFAULT now(),
    used_at    timestamptz
);
