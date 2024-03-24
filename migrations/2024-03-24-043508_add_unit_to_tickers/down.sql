-- This file should undo anything in `up.sql`

ALTER TABLE tickers DROP COLUMN unit VARCHAR NOT NULL;
