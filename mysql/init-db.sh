#!/bin/bash

MYSQL_PWD=root mysql -u root -e "CREATE DATABASE IF NOT EXISTS ivt_funds_api_development CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci;" mysql
MYSQL_PWD=root mysql -u root -e "CREATE DATABASE IF NOT EXISTS ivt_funds_api_test CHARACTER SET utf8mb3 COLLATE utf8mb3_general_ci;" mysql

