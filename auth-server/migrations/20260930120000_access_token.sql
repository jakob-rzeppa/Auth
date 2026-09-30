CREATE TABLE IF NOT EXISTS "access_tokens" (
    -- Only the hash of the token is stored, never the token itself.
    "token_hash"    VARCHAR(255)    PRIMARY KEY,

    "client_id"     UUID            NOT NULL,

    -- Issued at
    "iat"           TIMESTAMPTZ     NOT NULL,
    -- Expires at
    "exp"           TIMESTAMPTZ     NOT NULL,

    "scope"         TEXT            NOT NULL
);
