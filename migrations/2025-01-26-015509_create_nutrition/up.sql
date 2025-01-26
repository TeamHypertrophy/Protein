-- Your SQL goes here

-- Calories: Users will be able to log their calories
CREATE TABLE calorie_logs (
    "id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "date" TIMESTAMP NOT NULL DEFAULT (now()),
    "amount" INT NOT NULL, -- INT (CALORIES)
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

-- Protein: Users will be able to log their protein
CREATE TABLE protein_logs (
    "id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "date" TIMESTAMP NOT NULL DEFAULT (now()),
    "amount" INT NOT NULL, -- GRAMS
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

-- Sleep: Users will be able to log their sleep
CREATE TABLE sleep_logs (
    "id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "beginning" TIMESTAMP NOT NULL,
    "end" TIMESTAMP NOT NULL,
    "amount" INT NOT NULL DEFAULT (8), -- HOURS
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

-- Water: Users will be able to log their water intake
CREATE TABLE water_logs (
    "id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "date" TIMESTAMP NOT NULL DEFAULT (now()),
    "amount" INT NOT NULL, -- CUPS
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);
