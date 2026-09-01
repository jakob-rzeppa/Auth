CREATE TABLE IF NOT EXISTS "users_privileges" (
    "user_id"       UUID    NOT NULL,
    "privilege_id"  UUID    NOT NULL,
    PRIMARY KEY ("user_id", "privilege_id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);
