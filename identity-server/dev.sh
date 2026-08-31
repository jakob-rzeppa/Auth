#!/usr/bin/env bash
# Fast local dev loop: run identity-server natively with cargo watch against
# the Dockerized Postgres, instead of rebuilding the whole image on each change.
#
# Prereqs:
#   docker compose up -d identity-server-db identity-server-migrate
#   cargo install cargo-watch
set -euo pipefail
cd "$(dirname "$0")"

set -a
source .env
[ -f .env.local ] && source .env.local
set +a

export DATABASE_URL="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}"

cargo watch -x run
