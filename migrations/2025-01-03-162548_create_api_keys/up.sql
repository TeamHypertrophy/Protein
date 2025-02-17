-- Your SQL goes here
CREATE TYPE Status AS ENUM (
    'active',
    'revoked',
    'expired'
);

-- API Keys: Main Source of Authentication
CREATE TABLE IF NOT EXISTS api_keys (
    "key_id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    "api_key" UUID NOT NULL UNIQUE DEFAULT (gen_random_uuid()),
    "created_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "expires_at" TIMESTAMP NOT NULL DEFAULT (now() + interval '3 months'),
    "role" Role NOT NULL DEFAULT ('user'),
    "revoked_reason" TEXT NOT NULL DEFAULT (''),
    "status" Status NOT NULL DEFAULT ('active'),
    "quota" INT NOT NULL DEFAULT (0)
);

CREATE TRIGGER update_timestamp BEFORE UPDATE ON api_keys
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();
