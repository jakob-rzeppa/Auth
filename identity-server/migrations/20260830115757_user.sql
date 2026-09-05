CREATE TABLE IF NOT EXISTS "users" (
    "id"                        UUID            PRIMARY KEY,
    "email"                     VARCHAR(255)    NOT NULL UNIQUE,

    "password_hash"             VARCHAR(255)    NOT NULL,
    "has_temporary_password"    BOOLEAN         NOT NULL
);
