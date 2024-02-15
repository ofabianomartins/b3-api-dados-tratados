FROM mysql:8.0.34

ENV MYSQL_ROOT_PASSWORD=root
ENV MYSQL_USER=ivtfunds
ENV MYSQL_PASSWORD=ivtfunds

COPY init-db.sh /docker-entrypoint-initdb.d/
