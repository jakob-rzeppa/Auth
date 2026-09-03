# Setup

## Prerequisites

- Docker Compose v2.24+. The root `docker-compose.yml` uses `include:` with a per-include `env_file`, and `identity-server/e2e.sh` passes `--env-file` more than once; both need this version or newer. Check with `docker compose version`.

## First-time setup

Each service directory owns its own env files:

- `<service>/.env` — committed, holds safe placeholder values, acts as a fallback and development environment.
- `<service>/.env.local` — gitignored (`**/*.local`), holds your real local secrets, and overrides `.env` when present.

For `identity-server`, copy the template and fill in a real password:

```sh
cp identity-server/.env identity-server/.env.local
```

Then edit `identity-server/.env.local` and replace `POSTGRES_PASSWORD=change-me` with a real value.

## Compose file layout

`identity-server` splits its compose definition in three, because compose merges
`ports:` by appending - a port published in the base file cannot be removed by an
override, which would make a second stack impossible.

| File | Contents |
|---|---|
| `identity-server/docker-compose.yml` | Services, networks, volume, healthchecks. Publishes nothing. |
| `identity-server/docker-compose.dev.yml` | Host port publishing for local development. |
| `identity-server/docker-compose.e2e.yml` | The end-to-end test stack. Publishes nothing. |
| `identity-server/e2e-tests/` | The black-box HTTP test crate, run by `identity-server/e2e.sh`. |

The root `docker-compose.yml` includes the base file plus the dev override, so
everything below works from the repo root as usual.

### Networks

| Network | Services | Notes |
|---|---|---|
| `identity-server-backend` | db, migrate, app | `internal: true` - no egress, and the database is unreachable from outside the network |
| `identity-server-frontend` | app | Normal bridge network |

`identity-server` is the only service on both, so it is the only route to the
database. The dev override's `127.0.0.1:${DB_HOST_PORT}:5432` publish deliberately
bypasses this for local tooling; it is dev-only and absent from the test stack.

### Ports

Ports are configured in the env files, never hardcoded in compose:

| Variable | Meaning |
|---|---|
| `APP_PORT` | Port the app binds to inside its container (read by `Config`) |
| `APP_HOST_PORT` | Host port for the app - dev override only |
| `DB_HOST_PORT` | Host port for Postgres - dev override only |

`env_file:` on a service only sets variables *inside* the container; it does not
feed `${...}` interpolation in the compose file. The root `include:` therefore also
names `env_file: identity-server/.env`, and `identity-server/e2e.sh` passes `--env-file` explicitly.

## Running the stack

From the repo root:

```sh
docker compose up --build
```

This brings up, in order:

1. `identity-server-db` - Postgres, dedicated to identity-server. Published to the host on `127.0.0.1:${DB_HOST_PORT}` for local dev tooling; reachable from other containers as `identity-server-db`.
2. `identity-server-migrate` - runs `sqlx migrate run` once `identity-server-db` is healthy, then exits (exit code 0 on success).
3. `identity-server` - builds and starts once migrations complete, and reports healthy once `GET /health` answers.

## Development

Rebuilding the Docker image on every change (`docker compose watch`) recompiles the whole Rust dependency tree in release mode on each save, which is slow. Instead, run the app natively against the Dockerized Postgres:

```sh
docker compose up -d identity-server-db identity-server-migrate
cd identity-server && ./dev.sh
```

`dev.sh` requires `cargo-watch` (`cargo install cargo-watch`), reads `.env`/`.env.local` for Postgres credentials, and points `DATABASE_URL` at `localhost:5432` instead of the in-network `identity-server-db` hostname. It rebuilds and restarts on every source change, incrementally, in debug mode — much faster than a full container rebuild.

## Verifying

Confirm migrations applied:

```sh
docker compose exec identity-server-db psql -U identity_server -d identity_server -c '\dt'
```

You should see `users`, `privileges`, and `users_privileges` tables.

## End-to-end tests

`./e2e.sh` from `identity-server/` runs the black-box suite in
`identity-server/e2e-tests/` against a real, fully containerised stack:

```sh
cd identity-server && ./e2e.sh
```

It runs under its own compose project (`identity-server-e2e`) with its own
containers, network, and volume, and publishes no host ports - the suite reaches
the service at `${BASE_URL}` from inside the compose network. It therefore runs
happily while your dev stack is up.

Configuration comes from `identity-server/.env.test`, which is committed and holds
throwaway credentials. It is layered after `.env`/`.env.local`, so it never picks up
your local secrets.

The script tears the stack down with `down -v` on exit, and again *before* starting,
so a run that was hard-killed cannot leave a stale database behind. Every run starts
empty. `e2e.sh` exits with the test process's exit code.

The `e2e-tests` crate is deliberately separate from `identity-server`: it shares no
code with the service and keeps its client dependencies out of the service's
dependency graph. It has its own `Cargo.toml` and is not a workspace member.

## Adding a new service

Follow the same pattern as `identity-server`:

1. Add `<service>/docker-compose.yml` defining the service's own db, migration step, and app container — no other directory should reference it. Keep host ports out of it and put them in a `<service>/docker-compose.dev.yml` override.
2. Add `<service>/.env` (committed placeholder) and instruct contributors to create their own `<service>/.env.local`.
3. Add the service to the root `docker-compose.yml`:
   ```yaml
   include:
     - path:
         - <service>/docker-compose.yml
         - <service>/docker-compose.dev.yml
       env_file: <service>/.env
   ```
