// @generated automatically by Diesel CLI.

diesel::table! {
    repository (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 512]
        url -> Nullable<Varchar>,
        is_enabled -> Bool,
        #[max_length = 512]
        git_source -> Varchar,
        git_source_secret_key -> Nullable<Varchar>,
        #[max_length = 512]
        git_target -> Varchar,
        git_target_secret_key -> Nullable<Varchar>,
        git_clone_period_seconds -> Int4,
        last_clone_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    repository_logs (id) {
        id -> Uuid,
        repository_id -> Uuid,
        #[sql_name = "type"]
        #[max_length = 30]
        type_ -> Varchar,
        message -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user (id) {
        id -> Uuid,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 256]
        password_hash -> Nullable<Varchar>,
        #[max_length = 256]
        session_token -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(repository -> user (user_id));
diesel::joinable!(repository_logs -> repository (repository_id));

diesel::allow_tables_to_appear_in_same_query!(
    repository,
    repository_logs,
    user,
);
