-- This file should undo anything in `up.sql`


ALTER TABLE public.tickers DROP CONSTRAINT fk_segments_tickers;
ALTER TABLE public.tickers DROP COLUMN segment_id;
