#!/bin/bash

# Inicie o Puma
bundle exec puma -C config/puma.rb &

# Inicie o sidekiq
bundle exec sidekiq -C config/sidekiq.yml &

tail -f /dev/null
