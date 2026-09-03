#!/usr/bin/env bash
# Run the end-to-end suite against a real, fully containerised identity-server.
#
# The stack runs under its own compose project, so it coexists with the dev stack,
# and publishes no host ports - the suite talks to the service from inside the
# compose network. Everything is removed afterwards, volume included.
set -euo pipefail
cd "$(dirname "$0")"

PROJECT=identity-server-e2e
COMPOSE=(
  docker compose
  -p "$PROJECT"
  -f docker-compose.yml
  -f docker-compose.e2e.yml
  # Supplies ${APP_PORT} and friends for interpolation in the compose files.
  # Later files win, so .env.test overrides .env.
  --env-file .env
  --env-file .env.test
)

teardown() {
  "${COMPOSE[@]}" down -v --remove-orphans
}

# Clear anything a previously killed run left behind, so the database is always
# fresh even if that run never reached its trap.
teardown
trap teardown EXIT

"${COMPOSE[@]}" run --rm --build e2e
