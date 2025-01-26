-- Your SQL goes here

-- Trainer: Users will be able to follow trainers workout plans
CREATE TABLE trainers (
    "id" SERIAL PRIMARY KEY,
    "trainer_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "clients" UUID[] NOT NULL DEFAULT ('{}'),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);

-- Trainer Announcements: Trainers will be able to post announcements
CREATE TABLE trainer_announcements (
    "id" SERIAL PRIMARY KEY,
    "trainer_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "title" VARCHAR(100) NOT NULL,
    "content" TEXT NOT NULL DEFAULT (''),
    "created_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);
