-- Your SQL goes here

-- Calories: Users will be able to log their calories
CREATE TABLE IF NOT EXISTS calorie_logs (
    "log_id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    "date" TIMESTAMP NOT NULL DEFAULT (now()),
    "amount" INT NOT NULL, -- INT (CALORIES)
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

-- Protein: Users will be able to log their protein
CREATE TABLE IF NOT EXISTS protein_logs (
    "log_id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    "date" TIMESTAMP NOT NULL DEFAULT (now()),
    "amount" INT NOT NULL, -- GRAMS
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

-- Sleep: Users will be able to log their sleep
CREATE TABLE IF NOT EXISTS sleep_logs (
    "log_id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    "beginning" TIMESTAMP NOT NULL,
    "end" TIMESTAMP NOT NULL,
    "amount" INT NOT NULL DEFAULT (8), -- HOURS
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

-- Water: Users will be able to log their water intake
CREATE TABLE IF NOT EXISTS water_logs (
    "log_id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    "date" TIMESTAMP NOT NULL DEFAULT (now()),
    "amount" INT NOT NULL, -- CUPS
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

CREATE TRIGGER update_timestamp BEFORE UPDATE ON calorie_logs
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();

CREATE TRIGGER update_timestamp BEFORE UPDATE ON protein_logs
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();

CREATE TRIGGER update_timestamp BEFORE UPDATE ON sleep_logs
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();

CREATE TRIGGER update_timestamp BEFORE UPDATE ON water_logs
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();
