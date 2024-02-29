-- Your SQL goes here


CREATE TABLE IF NOT EXISTS public.theory_portfolios(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  index_id SERIAL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_theory_portfolios_tickers FOREIGN KEY(index_id) REFERENCES tickers(id) 
)

