-- Your SQL goes here

ALTER TABLE public.tickers ADD COLUMN segment_id SERIAL NOT NULL;
ALTER TABLE public.tickers
    ADD CONSTRAINT fk_segments_tickers FOREIGN KEY (segment_id) REFERENCES segments(id);
