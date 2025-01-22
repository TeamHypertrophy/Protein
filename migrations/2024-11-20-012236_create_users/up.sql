-- Your SQL goes here
CREATE TYPE Gender as ENUM (
    'male',
    'female'
);

CREATE TYPE PreferredWeight as ENUM (
    'kg',
    'lbs'
);

CREATE TYPE PreferredHeight as ENUM (
    'cm',
    'in'
);

CREATE TYPE Role as ENUM (
    'developer',
    'admin',
    'trainer',
    'user'
);

CREATE TABLE users (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "username" VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) UNIQUE NOT NULL,
    "password_updated_at" TIMESTAMP NOT NULL DEFAULT now(),
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "role" Role NOT NULL DEFAULT 'user',
    "ip_address" VARCHAR(255) NOT NULL
);

CREATE TABLE profile (
    "id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "first_name" VARCHAR(255) NOT NULL,
    "last_name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "age" INT NOT NULL,
    "weight" FLOAT NOT NULL DEFAULT '100.0', -- lbs
    "height" FLOAT NOT NULL DEFAULT '150.0', -- centimeters
    "gender" Gender NOT NULL,
    "preferred_weight_unit" PreferredWeight NOT NULL DEFAULT 'lbs',
    "preferred_height_unit" PreferredHeight NOT NULL DEFAULT 'cm'
);
