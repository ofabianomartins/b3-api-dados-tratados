-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.tickers(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  security_type VARCHAR NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  company_id SERIAL NOT NULL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_companies_tickers FOREIGN KEY(company_id) REFERENCES companies(id)
)
