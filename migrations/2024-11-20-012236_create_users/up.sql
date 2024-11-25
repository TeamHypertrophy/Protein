-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    is_dev BOOLEAN NOT NULL DEFAULT FALSE
)