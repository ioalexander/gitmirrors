CREATE TABLE public.repository (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL REFERENCES "user" (
        id
    ) ON DELETE CASCADE ON UPDATE CASCADE,
    name varchar(200) NOT NULL,
    url varchar(512),
    is_enabled boolean NOT NULL DEFAULT TRUE,
    git_source varchar(512) NOT NULL,
    git_source_secret_key varchar,
    git_target varchar(512) NOT NULL,
    git_target_secret_key varchar,
    git_clone_period_seconds int NOT NULL CHECK (git_clone_period_seconds > 0),
    last_clone_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),

    CONSTRAINT "repository_pkey" PRIMARY KEY ("id")
);

CREATE INDEX idx_repository_user_id ON repository (user_id);
