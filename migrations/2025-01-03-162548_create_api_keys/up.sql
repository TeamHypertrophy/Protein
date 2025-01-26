-- Your SQL goes here
CREATE TYPE Status AS ENUM (
    'active',
    'revoked',
    'expired'
);

-- API Keys: Main Source of Authentication
CREATE TABLE api_keys (
    "id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "api_key" UUID NOT NULL UNIQUE DEFAULT (gen_random_uuid()),
    "created_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "expires_at" TIMESTAMP NOT NULL DEFAULT (now() + interval '3 months'),
    "is_developer_key" BOOLEAN NOT NULL DEFAULT (FALSE),
    "revoked_reason" TEXT NOT NULL DEFAULT (''),
    "status" Status NOT NULL DEFAULT ('active'),
    "quota" INT NOT NULL DEFAULT (1000)
);
