// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ckg_session_participant_state"))]
    pub struct CkgSessionParticipantState;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ckg_session_state"))]
    pub struct CkgSessionState;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CkgSessionState;

    ckg_session (id) {
        id -> Int4,
        initiated_by -> Nullable<Int4>,
        threshold -> Int4,
        total_participants -> Int4,
        current_state -> CkgSessionState,
        ckg_session_timeout -> Int4,
        generated_public_key -> Varchar,
        created_on -> Timestamp,
        last_updated -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CkgSessionParticipantState;

    ckg_session_participant_values (id) {
        id -> Int4,
        ckg_session_id -> Nullable<Int4>,
        participant_id -> Nullable<Int4>,
        current_state -> Nullable<CkgSessionParticipantState>,
        session_public_keys -> Nullable<Bytea>,
        round1_data -> Nullable<Bytea>,
        round2_data -> Nullable<Bytea>,
        round3_data -> Nullable<Bytea>,
        last_updated -> Timestamp,
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
        user_id -> Nullable<Int4>,
        role_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
    }
}

diesel::joinable!(ckg_session -> users (initiated_by));
diesel::joinable!(ckg_session_participant_values -> ckg_session (ckg_session_id));
diesel::joinable!(ckg_session_participant_values -> users (participant_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    ckg_session,
    ckg_session_participant_values,
    roles,
    user_roles,
    users,
);
