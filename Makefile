SHELL := /bin/bash # Use bash syntax

rs:
	DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_development diesel migration run
	DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_development REDIS_URL=redis://127.0.0.1:6379 cargo run --bin server

job:
	DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_development diesel migration run
	DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_development REDIS_URL=redis://127.0.0.1:6379 cargo run --bin job

test:
	DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_test diesel migration run
	DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_test cargo test -- --test-threads=1

redis:
	docker exec -it ghostfoliodataredis redis-cli

insert_seed:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_development < ./seed/DEFAULT.sql

psql:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_development

psql_test:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_test
