# Setup

## Prerequisites

- Docker Compose v2.22+ (the root `docker-compose.yml` uses the `include:` directive, and `identity-server` uses `develop.watch`, both of which need this version or newer). Check with `docker compose version`.

## First-time setup

Each service directory owns its own env files:

- `<service>/.env` — committed, holds safe placeholder values, acts as a fallback and development environment.
- `<service>/.env.local` — gitignored (`**/*.local`), holds your real local secrets, and overrides `.env` when present.

For `identity-server`, copy the template and fill in a real password:

```sh
cp identity-server/.env identity-server/.env.local
```

Then edit `identity-server/.env.local` and replace `POSTGRES_PASSWORD=change-me` with a real value.

## Running the stack

From the repo root:

```sh
docker compose up --build
```

This brings up, in order:

1. `identity-server-db` — Postgres, dedicated to identity-server. Not reachable from the host, only from other containers.
2. `identity-server-migrate` — runs `sqlx migrate run` once `identity-server-db` is healthy, then exits (exit code 0 on success).
3. `identity-server` — builds and starts once migrations complete.

## Development

To have `identity-server` automatically rebuild and restart when its Rust source changes, run:

```sh
docker compose watch
```

This only watches `identity-server/src`, `identity-server/Cargo.toml`, and `identity-server/Cargo.lock`. Changes to migrations, `.env`/`.env.local`, or the Dockerfile/compose files are **not** watched — after editing those, restart the stack manually (`docker compose up --build`).

## Verifying

Confirm migrations applied:

```sh
docker compose exec identity-server-db psql -U identity_server -d identity_server -c '\dt'
```

You should see `users`, `privileges`, and `users_privileges` tables.

## Adding a new service

Follow the same pattern as `identity-server`:

1. Add `<service>/docker-compose.yml` defining the service's own db, migration step, and app container — no other directory should reference it.
2. Add `<service>/.env` (committed placeholder) and instruct contributors to create their own `<service>/.env.local`.
3. Add one line to the root `docker-compose.yml`:
   ```yaml
   include:
     - <service>/docker-compose.yml
   ```
