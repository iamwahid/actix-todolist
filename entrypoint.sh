#!/bin/bash

export MYSQL_PWD="${MYSQL_PASSWORD}"
until mysql -u $MYSQL_USER -h $MYSQL_HOST -P $MYSQL_PORT -D $MYSQL_DBNAME -e '\q'; do
>&2 echo "MySQL is still unavailable - sleeping"
sleep 1
done
>&2 echo "MySQL is up and running on port ${MYSQL_PORT}!"

export DATABASE_URL=mysql://${MYSQL_USER}:${MYSQL_PASSWORD}@${MYSQL_HOST}:${MYSQL_PORT}/${MYSQL_DBNAME}

mysql -h $MYSQL_HOST -u $MYSQL_USER -p$MYSQL_PASSWORD $MYSQL_DBNAME < db.sql

/app/zero2prod
