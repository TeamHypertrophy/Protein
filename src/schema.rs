// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "activitylevel"))]
    pub struct Activitylevel;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "diet"))]
    pub struct Diet;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "difficulty"))]
    pub struct Difficulty;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "equipment"))]
    pub struct Equipment;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "exercisetype"))]
    pub struct Exercisetype;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "fitnessgoal"))]
    pub struct Fitnessgoal;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "gender"))]
    pub struct Gender;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "musclegroup"))]
    pub struct Musclegroup;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "preferredheight"))]
    pub struct Preferredheight;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "preferredweight"))]
    pub struct Preferredweight;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "specialization"))]
    pub struct Specialization;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status"))]
    pub struct Status;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "userstatus"))]
    pub struct Userstatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "workoutinterval"))]
    pub struct Workoutinterval;
}

diesel::table! {
    api_key_logs (log_id) {
        log_id -> Int4,
        user_id -> Uuid,
        api_key -> Uuid,
        #[max_length = 10]
        method -> Varchar,
        #[max_length = 255]
        route -> Varchar,
        status_code -> Int4,
        #[max_length = 50]
        ip_address -> Varchar,
        #[max_length = 255]
        user_agent -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;
    use super::sql_types::Status;

    api_keys (key_id) {
        key_id -> Int4,
        user_id -> Uuid,
        api_key -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        expires_at -> Timestamp,
        role -> Role,
        revoked_reason -> Text,
        status -> Status,
        quota -> Int4,
    }
}

diesel::table! {
    calorie_logs (log_id) {
        log_id -> Int4,
        user_id -> Uuid,
        date -> Timestamp,
        amount -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Equipment;
    use super::sql_types::Difficulty;
    use super::sql_types::Musclegroup;
    use super::sql_types::Exercisetype;

    custom_exercises (exercise_id) {
        exercise_id -> Int8,
        user_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        equipment -> Equipment,
        difficulty -> Difficulty,
        muscle_group -> Musclegroup,
        sets -> Int4,
        reps -> Int4,
        rest_time -> Int4,
        exercise_type -> Exercisetype,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exercise_logs (log_id) {
        log_id -> Int4,
        user_id -> Uuid,
        exercise_id -> Int8,
        sets_completed -> Int4,
        reps_completed -> Int4,
        date -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Equipment;
    use super::sql_types::Difficulty;
    use super::sql_types::Musclegroup;
    use super::sql_types::Exercisetype;

    exercises (exercise_id) {
        exercise_id -> Int8,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        instructions -> Text,
        equipment -> Equipment,
        difficulty -> Difficulty,
        muscle_group -> Musclegroup,
        sets -> Int4,
        reps -> Int4,
        rest_time -> Int4,
        exercise_type -> Exercisetype,
        image_url -> Text,
        video_url -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Gender;
    use super::sql_types::Preferredweight;
    use super::sql_types::Preferredheight;
    use super::sql_types::Activitylevel;
    use super::sql_types::Fitnessgoal;
    use super::sql_types::Diet;

    profiles (profile_id) {
        profile_id -> Int4,
        user_id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        age -> Int4,
        weight -> Float8,
        height -> Float8,
        gender -> Gender,
        preferred_weight_unit -> Preferredweight,
        preferred_height_unit -> Preferredheight,
        public -> Bool,
        bio -> Text,
        streak -> Int4,
        avatar_url -> Text,
        activity_level -> Activitylevel,
        fitness_goal -> Fitnessgoal,
        diet -> Diet,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    protein_logs (log_id) {
        log_id -> Int4,
        user_id -> Uuid,
        date -> Timestamp,
        amount -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    sleep_logs (log_id) {
        log_id -> Int4,
        user_id -> Uuid,
        beginning -> Timestamp,
        end -> Timestamp,
        amount -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    trainer_announcements (announcement_id) {
        announcement_id -> Int4,
        trainer_id -> Uuid,
        #[max_length = 100]
        title -> Varchar,
        visibility -> Bool,
        content -> Text,
        pinned -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Specialization;

    trainers (trainer_id) {
        trainer_id -> Int4,
        user_id -> Uuid,
        clients -> Array<Nullable<Uuid>>,
        specialization -> Specialization,
        verified -> Bool,
        verified_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;
    use super::sql_types::Userstatus;

    users (user_id) {
        user_id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        email_verified -> Bool,
        email_verified_at -> Nullable<Timestamp>,
        email_verification_token -> Uuid,
        mfa_enabled -> Bool,
        #[max_length = 6]
        mfa_code -> Nullable<Varchar>,
        mfa_verified -> Bool,
        mfa_verification_token -> Uuid,
        mfa_code_expires_at -> Nullable<Timestamp>,
        password_updated_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login -> Timestamp,
        #[max_length = 50]
        last_login_ip -> Varchar,
        #[max_length = 50]
        ip_address -> Varchar,
        role -> Role,
        status -> Userstatus,
    }
}

diesel::table! {
    water_logs (log_id) {
        log_id -> Int4,
        user_id -> Uuid,
        date -> Timestamp,
        amount -> Int4,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    workout_logs (log_id) {
        log_id -> Int4,
        user_id -> Uuid,
        workout_id -> Uuid,
        date -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    workout_plan_logs (log_id) {
        log_id -> Int4,
        user_id -> Uuid,
        plan_id -> Uuid,
        date -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Workoutinterval;
    use super::sql_types::Fitnessgoal;
    use super::sql_types::Difficulty;

    workout_plans (plan_id) {
        plan_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        workouts -> Array<Nullable<Uuid>>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        start_time -> Timestamp,
        repeats -> Workoutinterval,
        goal -> Fitnessgoal,
        difficulty -> Difficulty,
        is_public -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Difficulty;

    workouts (workout_id) {
        workout_id -> Uuid,
        user_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Text,
        duration -> Int4,
        difficulty -> Difficulty,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        exercises -> Array<Nullable<Int8>>,
    }
}

diesel::joinable!(api_key_logs -> users (user_id));
diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(calorie_logs -> users (user_id));
diesel::joinable!(custom_exercises -> users (user_id));
diesel::joinable!(exercise_logs -> exercises (exercise_id));
diesel::joinable!(exercise_logs -> users (user_id));
diesel::joinable!(profiles -> users (user_id));
diesel::joinable!(protein_logs -> users (user_id));
diesel::joinable!(sleep_logs -> users (user_id));
diesel::joinable!(trainer_announcements -> users (trainer_id));
diesel::joinable!(trainers -> users (user_id));
diesel::joinable!(water_logs -> users (user_id));
diesel::joinable!(workout_logs -> users (user_id));
diesel::joinable!(workout_logs -> workouts (workout_id));
diesel::joinable!(workout_plan_logs -> users (user_id));
diesel::joinable!(workout_plan_logs -> workout_plans (plan_id));
diesel::joinable!(workout_plans -> users (user_id));
diesel::joinable!(workouts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_key_logs,
    api_keys,
    calorie_logs,
    custom_exercises,
    exercise_logs,
    exercises,
    profiles,
    protein_logs,
    sleep_logs,
    trainer_announcements,
    trainers,
    users,
    water_logs,
    workout_logs,
    workout_plan_logs,
    workout_plans,
    workouts,
);
