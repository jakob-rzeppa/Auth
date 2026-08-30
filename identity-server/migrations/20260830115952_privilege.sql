CREATE TABLE IF NOT EXISTS "privileges" (
    "id"        UUID            PRIMARY KEY,
    "name"      VARCHAR(255)    NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS "users_privileges" (
    "user_id"       UUID    NOT NULL,
    "privilege_id"  UUID    NOT NULL,
    PRIMARY KEY ("user_id", "privilege_id"),
    FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE,
    FOREIGN KEY ("privilege_id") REFERENCES "privileges" ("id") ON DELETE CASCADE
);
