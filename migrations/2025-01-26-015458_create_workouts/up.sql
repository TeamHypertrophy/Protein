-- Your SQL goes here

CREATE TYPE Difficulty AS ENUM (
    'beginner',
    'intermediate',
    'advanced'
);

CREATE TYPE WorkoutInterval AS ENUM (
    'daily',
    'weekly',
    'monthly'
);

-- Workouts: Users will be able to create their own workouts
CREATE TABLE workouts (
    "id" UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
    "name" VARCHAR(100) NOT NULL,
    "description" TEXT NOT NULL DEFAULT (''),
    "duration" INT NOT NULL DEFAULT ('30'), -- MINUTES
    "difficulty" Difficulty NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT now(),
    "updated_at" TIMESTAMP NOT NULL DEFAULT now(),
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "exercises" BIGINT[] NOT NULL DEFAULT ('{}')
);

-- Workout Plans: Users will be able to create workout plans
CREATE TABLE workout_plans (
    "id" UUID PRIMARY KEY DEFAULT (gen_random_uuid()),
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "name" VARCHAR(100) NOT NULL,
    "description" TEXT NOT NULL DEFAULT (''),
    "workouts" UUID[] NOT NULL DEFAULT ('{}'),
    "created_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now()),
    "start_time" TIMESTAMP NOT NULL DEFAULT (now() + interval '2 days'),
    "repeats" WorkoutInterval NOT NULL DEFAULT ('daily'), -- DAYS
    "goal" FitnessGoal NOT NULL DEFAULT ('muscle_gain'),
    "difficulty" Difficulty NOT NULL,
    "is_public" BOOLEAN NOT NULL DEFAULT (FALSE)
);

-- Workout Logs: Users will be able to log their workouts
CREATE TABLE workout_logs (
    "id" SERIAL PRIMARY KEY,
    "user_id" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "workout_id" UUID NOT NULL REFERENCES workouts(id) ON DELETE CASCADE,
    "date" TIMESTAMP NOT NULL DEFAULT (now()),
    "updated_at" TIMESTAMP NOT NULL DEFAULT (now())
);
