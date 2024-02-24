FROM postgres:14.2

RUN apt-get update
RUN apt-get install pv -y

USER postgres

ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres

COPY postgres.conf /etc/postgresql/postgresql.conf
COPY init-db.sh /docker-entrypoint-initdb.d/
