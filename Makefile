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

psql:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_development

psql_test:
	docker exec -it ghostfoliodatadb psql -d ghostfoliodata_test
