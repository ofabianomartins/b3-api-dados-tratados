-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.sectors(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS public.subsectors(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  sector_id SERIAL NOT NULL,
  CONSTRAINT fk_sector_subsectors FOREIGN KEY(sector_id) REFERENCES sectors(id) 
);

CREATE TABLE IF NOT EXISTS public.segments(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  subsector_id SERIAL NOT NULL,
  CONSTRAINT fk_subsector_segments FOREIGN KEY(subsector_id) REFERENCES subsectors(id) 
);
