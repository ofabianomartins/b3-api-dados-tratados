-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.companies(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  company_type VARCHAR NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW()
)
