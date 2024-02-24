#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
  CREATE DATABASE ghostfoliodata_development;
  CREATE DATABASE ghostfoliodata_test;
EOSQL

