-- Your SQL goes here

ALTER TABLE segments ADD COLUMN uuid UUID NOT NULL DEFAULT uuid_generate_v4();
ALTER TABLE segments ADD COLUMN created_at timestamp with time zone NOT NULL DEFAULT NOW();
ALTER TABLE segments ADD COLUMN updated_at timestamp with time zone NOT NULL DEFAULT NOW();

ALTER TABLE sectors ADD COLUMN uuid UUID NOT NULL DEFAULT uuid_generate_v4();
ALTER TABLE sectors ADD COLUMN created_at timestamp with time zone NOT NULL DEFAULT NOW();
ALTER TABLE sectors ADD COLUMN updated_at timestamp with time zone NOT NULL DEFAULT NOW();

ALTER TABLE subsectors ADD COLUMN uuid UUID NOT NULL DEFAULT uuid_generate_v4();
ALTER TABLE subsectors ADD COLUMN created_at timestamp with time zone NOT NULL DEFAULT NOW();
ALTER TABLE subsectors ADD COLUMN updated_at timestamp with time zone NOT NULL DEFAULT NOW();
