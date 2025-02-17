-- Your SQL goes here

-- Trainer: Users will be able to follow trainers workout plans
CREATE TABLE IF NOT EXISTS trainers (
    "trainer_id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    "clients" UUID[] NOT NULL DEFAULT ('{}'),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

-- Trainer Announcements: Trainers will be able to post announcements
CREATE TABLE IF NOT EXISTS trainer_announcements (
    "announcement_id" SERIAL PRIMARY KEY,
    "trainer_id" UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    "title" VARCHAR(100) NOT NULL,
    "content" TEXT NOT NULL DEFAULT (''),
    "created_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

CREATE TRIGGER update_timestamp BEFORE UPDATE ON trainers
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();

CREATE TRIGGER update_timestamp BEFORE UPDATE ON trainer_announcements
FOR EACH ROW EXECUTE PROCEDURE modify_updated_at();
