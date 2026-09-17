#!/usr/bin/env bash
# Fast local dev loop: run auth-server natively with cargo watch against a
# Dockerized Redis, instead of rebuilding the whole image on each change.
#
# Brings up (and tears down on exit) just Redis under its own compose
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

teardown() {
  "${COMPOSE[@]}" down --remove-orphans
}
trap teardown EXIT

"${COMPOSE[@]}" up -d auth-server-redis

cargo watch -x run
