-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.calendars(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  uuid VARCHAR NOT NULL UNIQUE,
  created_at timestamp with time zone NOT NULL,
  updated_at timestamp with time zone NOT NULL
)
