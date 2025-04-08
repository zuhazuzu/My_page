# syntax=docker/dockerfile:1.0.0-experimental
FROM postgres:latest
ENV POSTGRES_DB=tyotiedot
ENV POSTGRES_USER=postgres
RUN --mount=type=secret,id=postgres_password cat /run/secrets/postgres_password > /tmp/postgres_password
COPY init.sql /docker-entrypoint-initdb.d/
