// @generated automatically by Diesel CLI.

diesel::table! {
    ckg_session_participant_values (id) {
        id -> Int4,
        ckg_session_id -> Int4,
        participant_id -> Int4,
        current_state -> Nullable<Varchar>,
        session_public_keys -> Nullable<Bytea>,
        round1_data -> Nullable<Bytea>,
        round2_data -> Nullable<Bytea>,
        round3_data -> Nullable<Bytea>,
        updated_at -> Timestamp,
        created_on -> Timestamp,
    }
}

diesel::table! {
    ckg_sessions (id) {
        id -> Int4,
        initiated_by -> Int4,
        threshold -> Int4,
        total_participants -> Int4,
        current_state -> Varchar,
        ckg_session_timeout -> Int4,
        generated_public_key -> Varchar,
        updated_at -> Timestamp,
        created_on -> Timestamp,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
    }
}

diesel::joinable!(ckg_session_participant_values -> ckg_sessions (ckg_session_id));
diesel::joinable!(ckg_session_participant_values -> users (participant_id));
diesel::joinable!(ckg_sessions -> users (initiated_by));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    ckg_session_participant_values,
    ckg_sessions,
    roles,
    user_roles,
    users,
);
