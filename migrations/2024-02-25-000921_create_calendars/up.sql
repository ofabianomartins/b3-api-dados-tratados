-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS public.calendars(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4()
--  created_at timestamp with time zone NOT NULL,
--  updated_at timestamp with time zone NOT NULL
)
