-- Your SQL goes here

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR NOT NULL,
    is_dev BOOLEAN NOT NULL DEFAULT FALSE
)