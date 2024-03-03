-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.tickers(
  id SERIAL PRIMARY KEY, 
  symbol VARCHAR NOT NULL UNIQUE,
  security_type VARCHAR NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  company_id SERIAL NOT NULL,
  calendar_id SERIAL NOT NULL,
  currency_id SERIAL NOT NULL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_companies_tickers FOREIGN KEY(company_id) REFERENCES companies(id),
  CONSTRAINT fk_calendars_tickers FOREIGN KEY(calendar_id) REFERENCES calendars(id),
  CONSTRAINT fk_currencies_tickers FOREIGN KEY(currency_id) REFERENCES currencies(id)
)
