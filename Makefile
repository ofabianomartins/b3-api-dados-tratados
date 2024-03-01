SHELL := /bin/bash # Use bash syntax

build:
	docker-compose build app test

rs:
	docker-compose run --rm --service-ports app

dev:
	docker-compose run --rm app bash

rt:
	docker-compose run --rm test bash

down:
	docker-compose down

test:
	export DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ghostfoliodata_test
	cargo test -- --test-threads=1

psql:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_development

psql_test:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_test
