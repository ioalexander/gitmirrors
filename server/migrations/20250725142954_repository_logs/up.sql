CREATE TABLE public.repository_logs (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    repository_id uuid NOT NULL REFERENCES repository (
        id
    ) ON DELETE CASCADE ON UPDATE CASCADE,
    type varchar(30) NOT NULL,
    message text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),

    CONSTRAINT repository_logs_pkey PRIMARY KEY (id)
);

CREATE INDEX idx_repository_logs_repository_id ON repository_logs (
    repository_id
);
