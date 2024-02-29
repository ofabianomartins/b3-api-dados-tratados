-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.theory_portfolio_transactions(
  id SERIAL PRIMARY KEY, 
  date DATE NOT NULL,  
  quantity DECIMAL(25,8) NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  ticker_id SERIAL,
  theory_portfolio_id SERIAL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_theory_portfolio_transactions_tickers FOREIGN KEY(ticker_id) REFERENCES tickers(id), 
  CONSTRAINT fk_theory_portfolio_transactions_theory_portfolios FOREIGN KEY(theory_portfolio_id) REFERENCES theory_portfolios(id) 
)
