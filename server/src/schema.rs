// @generated automatically by Diesel CLI.

diesel::table! {
    user (id) {
        id -> Uuid,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 256]
        password_hash -> Varchar,
        #[max_length = 256]
        session_token -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
