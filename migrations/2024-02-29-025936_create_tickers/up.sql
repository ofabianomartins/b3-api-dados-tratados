-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.tickers(
  id SERIAL PRIMARY KEY, 
  symbol VARCHAR NOT NULL UNIQUE,
  security_type VARCHAR NOT NULL,
  creation_date DATE NOT NULL,  
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  company_id SERIAL NOT NULL,
  calendar_id SERIAL NOT NULL,
  currency_id SERIAL NOT NULL,
  ticker_id SERIAL NULL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_tickers_tickers FOREIGN KEY(ticker_id) REFERENCES tickers(id),
  CONSTRAINT fk_companies_tickers FOREIGN KEY(company_id) REFERENCES companies(id),
  CONSTRAINT fk_calendars_tickers FOREIGN KEY(calendar_id) REFERENCES calendars(id),
  CONSTRAINT fk_currencies_tickers FOREIGN KEY(currency_id) REFERENCES currencies(id)
)
