-- This file should undo anything in `up.sql`
DROP TYPE Gender;

DROP TYPE PreferredWeight;

DROP TYPE PreferredHeight;

DROP TYPE Role;

DROP TYPE FitnessGoal;

DROP TABLE if exists users cascade;

DROP TABLE if exists profile cascade;
