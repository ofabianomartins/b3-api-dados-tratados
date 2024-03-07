-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.indicators(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  symbol VARCHAR NOT NULL UNIQUE,
  description VARCHAR NOT NULL UNIQUE,
  indicator_type VARCHAR NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS public.indicator_values(
  id SERIAL PRIMARY KEY, 
  indicator_id SERIAL NOT NULL,
  company_id SERIAL NOT NULL,
  date DATE NOT NULL,  
  close DECIMAL(25,8) NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_quotes_indicators FOREIGN KEY(indicator_id) REFERENCES indicators(id),
  CONSTRAINT fk_quotes_companies FOREIGN KEY(company_id) REFERENCES companies(id) 
);
