# frozen_string_literal: true

source 'https://rubygems.org'
git_source(:github) { |repo| "https://github.com/#{repo}.git" }

ruby '3.1.0'

gem 'rackup'
gem 'sinatra', '~> 4.0'
gem 'puma'
gem 'rake'
gem 'sidekiq'

group :development, :test do
  gem 'bullet'
  gem 'bundler-audit'
  gem 'debug', platforms: %i[mri mingw x64_mingw]
  gem 'guard'
  gem 'guard-rubocop'
  gem 'listen'
  gem 'mock_redis'
  gem 'pry'
  gem 'rubocop-ast', require: false
  gem 'rubocop-performance', require: false
  gem 'rubocop-rspec', require: false
end

group :test do
  gem 'factory_bot', '~> 6.4'
  gem 'rspec', '~> 3.13'
  gem 'simplecov', require: false
  gem 'vcr', '~> 6.2'
  gem 'webmock', '~> 3.18'
end
