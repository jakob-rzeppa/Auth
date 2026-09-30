#!/usr/bin/env bash
# Fast local dev loop: run auth-server natively with cargo watch against a
# Dockerized Redis and Postgres, instead of rebuilding the whole image on each change.
#
# Brings up (and tears down on exit) just Redis, Postgres and its migrations under its own compose
# project, so this coexists with any other auth-server stack. The app itself
# runs on the host via `cargo watch`.
#
# Prereqs:
#   cargo install cargo-watch
set -euo pipefail
cd "$(dirname "$0")"

PROJECT=auth-server-dev
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

export REDIS_URL="redis://127.0.0.1:${REDIS_HOST_PORT}"
export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:${DB_HOST_PORT}/${POSTGRES_DB}"

teardown() {
  "${COMPOSE[@]}" down --remove-orphans
}
trap teardown EXIT

"${COMPOSE[@]}" up -d --build auth-server-redis auth-server-db auth-server-migrate

cargo watch -x run
