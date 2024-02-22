SHELL := /bin/bash # Use bash syntax

rs:
	docker-compose run --rm --service-ports app cargo run

dev:
	docker-compose run --rm app bash

rt:
	docker-compose run --rm test bash

down:
	docker-compose down
