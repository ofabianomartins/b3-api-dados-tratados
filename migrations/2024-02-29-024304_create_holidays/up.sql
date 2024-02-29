-- Your SQL goes here

CREATE TABLE IF NOT EXISTS public.holidays(
  id SERIAL PRIMARY KEY, 
  name VARCHAR NOT NULL UNIQUE,
  date DATE NOT NULL,  
  uuid UUID NOT NULL DEFAULT uuid_generate_v4(),
  calendar_id SERIAL,
  created_at timestamp with time zone NOT NULL DEFAULT NOW(),
  updated_at timestamp with time zone NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_holiday_calendars FOREIGN KEY(calendar_id) REFERENCES calendars(id) 
)
