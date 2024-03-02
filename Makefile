SHELL := /bin/bash # Use bash syntax

rs:
	export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_development
	cargo run --bin server

job:
	export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_development
	cargo run --bin job

test:
	DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_test cargo test -- --test-threads=1

psql:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_development

psql_test:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_test
