CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE public.user (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    username character varying(32) NOT NULL,
    password_hash character varying(256) NOT NULL,
    session_token character varying(256) NOT NULL,
    created_at timestamp(6) with time zone NOT NULL DEFAULT current_timestamp,
    updated_at timestamp(6) with time zone NOT NULL DEFAULT current_timestamp,
    CONSTRAINT "user_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "UQ_user_username" UNIQUE ("username")
);
