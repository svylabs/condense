// @generated automatically by Diesel CLI.

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

diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    user_roles,
    users,
);
