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

CREATE TYPE UserStatus AS ENUM (
    'active',
    'pending'
);

CREATE TYPE ActivityLevel AS ENUM (
    'light',
    'moderate',
    'very'
    'extremely'
);

CREATE TYPE Diet AS ENUM (
    'vegetarian',
    'vegan',
    'keto',
    'anything'
);

-- Users: User Accounts
CREATE TABLE IF NOT EXISTS users (
    "user_id" UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
    "username" VARCHAR(255) UNIQUE NOT NULL,
    "password" VARCHAR(255) UNIQUE NOT NULL,
    "email" VARCHAR(255) UNIQUE NOT NULL,
    "email_verified" BOOLEAN NOT NULL DEFAULT (FALSE),
    "email_verified_at" TIMESTAMP,
    "email_verification_token" UUID NOT NULL DEFAULT (gen_random_uuid()),
    "mfa_enabled" BOOLEAN NOT NULL DEFAULT (FALSE),
    "mfa_code" VARCHAR(6),
    "mfa_verified" BOOLEAN NOT NULL DEFAULT (FALSE),
    "mfa_verification_token" UUID NOT NULL DEFAULT (gen_random_uuid()),
    "mfa_code_expires_at" TIMESTAMP,
    "password_updated_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "created_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "last_login" TIMESTAMP NOT NULL DEFAULT (now()),
    "last_login_ip" VARCHAR(50) NOT NULL,
    "ip_address" VARCHAR(50) NOT NULL,
    "role" Role NOT NULL DEFAULT ('user'),
    "status" UserStatus NOT NULL DEFAULT ('pending')
);

-- Profile: User Profiles
CREATE TABLE IF NOT EXISTS profiles (
    "profile_id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    "first_name" VARCHAR(255) NOT NULL,
    "last_name" VARCHAR(255) NOT NULL,
    "age" INT NOT NULL,
    "weight" FLOAT NOT NULL DEFAULT ('100.0'), -- lbs
    "height" FLOAT NOT NULL DEFAULT ('150.0'), -- centimeters
    "gender" Gender NOT NULL,
    "preferred_weight_unit" PreferredWeight NOT NULL DEFAULT ('lbs'),
    "preferred_height_unit" PreferredHeight NOT NULL DEFAULT ('cm'),
    "public" BOOLEAN NOT NULL DEFAULT (FALSE),
    "bio" TEXT NOT NULL DEFAULT (''),
    "streak" INT NOT NULL DEFAULT (0),
    "avatar_url" TEXT NOT NULL DEFAULT (''),
    "activity_level" ActivityLevel NOT NULL DEFAULT ('moderate'),
    "fitness_goal" FitnessGoal NOT NULL DEFAULT ('muscle_gain'),
    "diet" Diet NOT NULL DEFAULT ('anything'),
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now()
);

CREATE FUNCTION modify_updated_at() RETURNS TRIGGER
LANGUAGE PLPGSQL AS
$$
BEGIN
   NEW.updated_at := now();
   RETURN NEW;
END;
$$;

CREATE TRIGGER update_timestamp BEFORE UPDATE ON users
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();

CREATE TRIGGER update_timestamp BEFORE UPDATE ON profiles
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();
