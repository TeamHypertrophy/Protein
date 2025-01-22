// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "gender"))]
    pub struct Gender;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "preferredheight"))]
    pub struct Preferredheight;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "preferredweight"))]
    pub struct Preferredweight;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    api_keys (id) {
        id -> Int4,
        user_id -> Uuid,
        api_key -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_developer_key -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Gender;
    use super::sql_types::Preferredweight;
    use super::sql_types::Preferredheight;

    profile (id) {
        id -> Int4,
        user_id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        age -> Int4,
        weight -> Float8,
        height -> Float8,
        gender -> Gender,
        preferred_weight_unit -> Preferredweight,
        preferred_height_unit -> Preferredheight,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        password_updated_at -> Timestamp,
        created_at -> Timestamp,
        role -> Role,
        #[max_length = 255]
        ip_address -> Varchar,
    }
}

diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(profile -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(api_keys, profile, users,);
