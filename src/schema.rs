// @generated automatically by Diesel CLI.

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
    users (id) {
        id -> Uuid,
        username -> Varchar,
        is_dev -> Bool,
    }
}

diesel::joinable!(api_keys -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(api_keys, users,);
