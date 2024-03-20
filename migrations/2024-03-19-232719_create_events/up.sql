-- Your SQL goes here

CREATE TABLE IF NOT EXISTS events(
  id SERIAL PRIMARY KEY, 
  ticker_id SERIAL,
  date DATE NOT NULL,  
  ex_date DATE NOT NULL,  
  liquidation_date DATE NOT NULL,  
  type VARCHAR NOT NULL,
  factor DECIMAL(25,8) NOT NULL,
  strike DECIMAL(25,8),
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_events_tickers FOREIGN KEY(ticker_id) REFERENCES tickers(id) 
)
