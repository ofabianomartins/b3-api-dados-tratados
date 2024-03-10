-- Your SQL goes here


CREATE TABLE IF NOT EXISTS public.clients(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS public.client_positions(
  id SERIAL PRIMARY KEY, 
  company_id SERIAL NOT NULL,
  client_id SERIAL NOT NULL,
  date DATE NOT NULL,  
  side VARCHAR NOT NULL,
  value DECIMAL(25,8) NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_companies_asset_positions FOREIGN KEY(company_id) REFERENCES companies(id),
  CONSTRAINT fk_assets_asset_positions FOREIGN KEY(client_id) REFERENCES clients(id)
);

CREATE TABLE IF NOT EXISTS public.assets(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS public.asset_positions(
  id SERIAL PRIMARY KEY, 
  company_id SERIAL NOT NULL,
  asset_id SERIAL NOT NULL,
  date DATE NOT NULL,  
  side VARCHAR NOT NULL,
  quantity DECIMAL(25,8) NOT NULL,
  price DECIMAL(25,8) NOT NULL,
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_companies_asset_positions FOREIGN KEY(company_id) REFERENCES companies(id),
  CONSTRAINT fk_assets_asset_positions FOREIGN KEY(asset_id) REFERENCES assets(id)
);
