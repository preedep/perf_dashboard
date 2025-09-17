#!/bin/bash
# Run schema.sql on local Postgres using env vars from .env

set -e

# Load env vars
if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
fi

psql "host=localhost port=5432 user=$PG_USER password=$PG_PASSWORD dbname=$PG_DB" -f schema.sql
