-- Your SQL goes here
CREATE TYPE Gender AS ENUM (
    'male',
    'female'
);

CREATE TYPE PreferredWeight AS ENUM (
    'kg',
    'lbs'
);

CREATE TYPE PreferredHeight AS ENUM (
    'cm',
    'in'
);

CREATE TYPE Role AS ENUM (
    'developer',
    'admin',
    'trainer',
    'user'
);

CREATE TYPE FitnessGoal AS ENUM (
    'weight_loss',
    'muscle_gain',
    'maintenance',
    'endurance',
    'strength'
);

-- Users: Users will be able to create an account
CREATE TABLE users (
    "id" UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
    "username" VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) UNIQUE NOT NULL,
    "password_updated_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "created_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "role" Role NOT NULL DEFAULT ('user'),
    "ip_address" VARCHAR(255) NOT NULL
);

-- Profile: Users will be able to create a profile
CREATE TABLE profiles (
    "id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "first_name" VARCHAR(255) NOT NULL,
    "last_name" VARCHAR(255) NOT NULL,
    "email" VARCHAR(255) NOT NULL,
    "age" INT NOT NULL,
    "weight" FLOAT NOT NULL DEFAULT ('100.0'), -- lbs
    "height" FLOAT NOT NULL DEFAULT ('150.0'), -- centimeters
    "gender" Gender NOT NULL,
    "preferred_weight_unit" PreferredWeight NOT NULL DEFAULT ('lbs'),
    "preferred_height_unit" PreferredHeight NOT NULL DEFAULT ('cm'),
    "public" BOOLEAN NOT NULL DEFAULT (FALSE),
    "bio" TEXT NOT NULL DEFAULT (''),
    "avatar_url" TEXT NOT NULL DEFAULT (''),
    "fitness_goal" FitnessGoal NOT NULL DEFAULT ('muscle_gain'),
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now()
);
