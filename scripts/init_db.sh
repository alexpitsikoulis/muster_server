#!/usr/bin/env bash
set -x
set -eo pipefail

export DOCKER_DEFAULT_PLATFORM=linux/amd64

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "  cargo install --version '~0.6' sqlx-cli \
--no-default-features --features rustls,postgres"
    echo >&2 "to install it."
    exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:=password}
DB_NAME=${POSTGRES_DB:=muttr}
DB_PORT=${POSTGRES_PORT:=5432}

if docker ps | grep postgres-muttr; then
    continue
elif docker ps -a | grep postgres-muttr; then
    docker start postgres-muttr
else
    docker run \
        --name postgres-muttr \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -e POSTGRES_PORT=${DB_PORT} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
fi

# until psql -h localhost --user postgres -c "\q" ; do
#     >&2 echo "Postgres is still unavailable - sleeping"
#     sleep 1
# done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create
sqlx migrate run