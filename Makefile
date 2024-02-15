SHELL := /bin/bash # Use bash syntax

build:
	docker-compose build app test

rs:
	docker-compose run --rm --service-ports app bundle exec rackup -p 3000

rkiq:
	docker-compose run --rm app bundle exec sidekiq -C config/sidekiq.yml

migrate:
	docker-compose run --rm app rake db:migrate

rkafka:
	docker-compose run --rm --service-ports kafka bundle exec karafka server

dev:
	docker-compose run --rm app bash

rt:
	docker-compose run --rm test bash

down:
	docker-compose down

mariadb:
	docker exec -it ivtfundsdb mysql -u root -p  mysql
