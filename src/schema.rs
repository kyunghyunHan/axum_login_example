// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Varchar,
        user_pw -> Varchar,
        img -> Varchar,
    }
}
