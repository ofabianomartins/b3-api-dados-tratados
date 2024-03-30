-- This file should undo anything in `up.sql`

ALTER TABLE segments DROP COLUMN created_at;
ALTER TABLE segments DROP COLUMN updated_at;
ALTER TABLE segments DROP COLUMN uuid;

ALTER TABLE sectors DROP COLUMN created_at;
ALTER TABLE sectors DROP COLUMN updated_at;
ALTER TABLE sectors DROP COLUMN uuid;

ALTER TABLE subsectors DROP COLUMN created_at;
ALTER TABLE subsectors DROP COLUMN updated_at;
ALTER TABLE subsectors DROP COLUMN uuid;
