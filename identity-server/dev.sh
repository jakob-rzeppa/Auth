#!/usr/bin/env bash
# Fast local dev loop: run identity-server natively with cargo watch against a
# Dockerized Postgres, instead of rebuilding the whole image on each change.
#
# Brings up (and tears down on exit) just the database and its migrations under
# their own compose project, so this coexists with any other identity-server
# stack. The app itself runs on the host via `cargo watch`.
#
# Prereqs:
#   cargo install cargo-watch
set -euo pipefail
cd "$(dirname "$0")"

PROJECT=identity-server-dev
COMPOSE=(
  docker compose
  -p "$PROJECT"
  -f docker-compose.yml
  -f docker-compose.dev.yml
  --env-file .env
)

set -a
source .env
[ -f .env.local ] && source .env.local
set +a

export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${DB_HOST_PORT}/${POSTGRES_DB}"

teardown() {
  "${COMPOSE[@]}" down --remove-orphans
}
trap teardown EXIT

"${COMPOSE[@]}" up -d --build identity-server-db identity-server-migrate

cargo watch -x run
