#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v mysql)" ]; then
echo >&2 "Error: mysql is not installed."
exit 1
fi
if ! [ -x "$(command -v sqlx)" ]; then
echo >&2 "Error: sqlx is not installed."
echo >&2 "Use:"
echo >&2 " cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
echo >&2 "to install it."
exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${MYSQL_USER:=xxxx}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${MYSQL_PASSWORD:=xxxxx}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${MYSQL_DBNAME:=todo4}"
# Check if a custom port has been set, otherwise default to '5432'
DB_HOST="${MYSQL_HOST:=127.0.0.1}"
DB_PORT="${MYSQL_PORT:=3306}"

if [[ -z "${SKIP_DOCKER}" ]]
then
    # Launch postgres using Docker
    docker run \
    -e MYSQL_ROOT_PASSWORD=${DB_PASSWORD} \
    -e MYSQL_USER=${DB_USER} \
    -e MYSQL_PASSWORD=${DB_PASSWORD} \
    -e MYSQL_DATABASE=${DB_NAME} \
    -p "${DB_PORT}":3306 \
    -d mysql:5.7 \
    --default-authentication-plugin=mysql_native_password
    # --max_connections=1000 \
    # ^ Increased maximum number of connections for testing purposes
fi

# Keep pinging MySQL until it's ready to accept commands
export MYSQL_PWD="${DB_PASSWORD}"
until mysql -u $DB_USER -h $DB_HOST -P $DB_PORT -D $DB_NAME -e '\q'; do
>&2 echo "MySQL is still unavailable - sleeping"
sleep 1
done
>&2 echo "MySQL is up and running on port ${DB_PORT}!"
export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
# sqlx database create