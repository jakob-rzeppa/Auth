# identity-server end-to-end test stack

**Date:** 2026-09-03
**Status:** Approved design, not yet implemented

## Goal

Run functional tests against the real, fully containerised identity-server stack —
app, migrations, and Postgres — with three guarantees:

1. **Isolated.** The test stack runs at the same time as the dev stack without
   colliding on container names, host ports, or volumes.
2. **Clean.** Every run starts from an empty database.
3. **Ephemeral.** After a run, no containers, networks, or volumes remain.

Tests run inside the compose network. The stack publishes no host ports.

## Non-goals

- `auth-server` is untouched. It has no compose service today.
- Unit and integration tests inside the `identity-server` crate are unaffected.
  This design adds a black-box layer above them, not a replacement.
- No CI wiring. `e2e.sh` is written so CI can call it, but adding a workflow is
  separate work.

## Architecture

### Networks

`identity-server/docker-compose.yml` gains two networks:

| Network | Services | Notes |
|---|---|---|
| `identity-server-backend` | `identity-server-db`, `identity-server-migrate`, `identity-server` | `internal: true` — no egress, unreachable from outside the network |
| `identity-server-frontend` | `identity-server`, (`e2e` in the test stack) | Normal bridge network |

`identity-server` is the only service on both networks, so it is the only path to
the database. `internal: true` also denies the database container internet access.

### Compose file layout

The base file describes topology only. Everything host-facing moves to overrides,
because **compose merges `ports:` by appending — an override cannot remove a port
published in the base file.**

| File | Contents |
|---|---|
| `identity-server/docker-compose.yml` | Services, networks, volume, healthchecks. No `ports:`. |
| `identity-server/docker-compose.dev.yml` | Publishes the DB and app ports for local development. |
| `identity-server/docker-compose.e2e.yml` | Adds the `e2e` service. Publishes nothing. |
| `docker-compose.yml` (root) | `include:`s the base file **and** the dev override, so `docker compose up` from the root behaves exactly as it does today. |

`container_name:` is removed from all three services. Container names are global to
the Docker daemon, so pinning them makes a second stack impossible. Services remain
addressable by service name on their networks, which is what `DATABASE_URL` already
relies on.

### Configuration: no hardcoded ports

Ports move into env files. Compose resolves them in two distinct ways, and the
difference matters:

- **`env_file:`** on a service sets variables *inside the container*. It does **not**
  feed `${...}` interpolation in the compose file.
- **Interpolation** (`${APP_HOST_PORT}` in a `ports:` entry) reads from the shell
  environment or from a project-level env file supplied via `include: env_file:` or
  the `--env-file` CLI flag.

Both paths are therefore wired explicitly.

Variables:

| Variable | Meaning | `.env` | `.env.test` |
|---|---|---|---|
| `APP_PORT` | Port the app binds inside its container | `8080` | `8080` |
| `APP_HOST_PORT` | Host port for the app (dev override only) | `8080` | unused |
| `DB_HOST_PORT` | Host port for Postgres (dev override only) | `5432` | unused |
| `POSTGRES_USER` / `POSTGRES_PASSWORD` / `POSTGRES_DB` | Existing | existing values | test-only values |
| `DATABASE_URL` | Existing, in-network host | existing | same shape |
| `BASE_URL` | Target for the e2e suite | unused | `http://identity-server:${APP_PORT}` |

`identity-server/.env.test` is committed alongside `.env` and holds test-stack
values. It is listed **after** `.env` and `.env.local` in the `e2e` override's
`env_file:` lists so its values win, which keeps the test stack independent of a
developer's `.env.local` secrets.

Root compose supplies the base+dev interpolation env file:

```yaml
include:
  - path:
      - identity-server/docker-compose.yml
      - identity-server/docker-compose.dev.yml
    env_file: identity-server/.env
```

The e2e path supplies its own on the command line:

```sh
--env-file identity-server/.env --env-file identity-server/.env.test
```

Later `--env-file` flags override earlier ones.

### Application changes

**Port from config.** `identity-server/src/main.rs:19` hardcodes `8080`.
`Config` (`src/config.rs`) gains an `app_port` field read from `APP_PORT`, and
`main` binds `([0, 0, 0, 0], CONFIG.app_port())`.

**Health endpoint.** A new `GET /health` returns `200 OK` with a small JSON body.
It does not touch the database — it answers "is the HTTP server accepting
requests", which is exactly what the compose healthcheck gates on. It is registered
at the top level in `src/api/mod.rs`, outside `/users`.

**Healthcheck.** `identity-server` gains:

```yaml
healthcheck:
  test: ["CMD", "curl", "-fsS", "http://localhost:${APP_PORT}/health"]
  interval: 5s
  timeout: 3s
  retries: 10
  start_period: 5s
```

The runtime stage is `debian:bookworm-slim`, which has no `curl` or `wget`, so the
Dockerfile's `runtime` stage installs `curl`. This is the smallest change that makes
the healthcheck work; the alternative — a health-probe mode in the Rust binary — was
rejected as more production code for less clarity.

With this in place the `e2e` service can depend on
`identity-server: { condition: service_healthy }`, so the suite never races startup
and needs no retry loop of its own.

### Test crate

A new top-level crate `e2e-tests/`, added to the workspace. It is deliberately not
`identity-server/tests/`: that would put `reqwest` and `tokio` into identity-server's
dependency graph, and a black-box HTTP client is not part of the service.

- Dependencies: `reqwest` (json), `tokio` (macros, rt-multi-thread), `serde_json`, `uuid`.
- Reads `BASE_URL` from the environment; panics with a clear message if unset.
- Tests are `#[tokio::test]` functions issuing real HTTP requests.

Initial coverage, mirroring the current API surface in `src/api/users/`:

- `POST /users` → `201 Created`, body `{"id": ...}`
- `GET /users/{id}` after create → `200 OK`, matching email
- `PATCH /users/{id}` updates, and the change is visible on a subsequent `GET`
- `DELETE /users/{id}` → `204 No Content`, then `GET` → `404`
- `GET /users/{not-a-uuid}` → `400`, body `{"error": "...", "error_description": "..."}`
- `GET /users/{unknown-uuid}` → `404`
- `POST /users` with a duplicate email → `409 Conflict`, error code `email_already_exists`
- `GET /health` → `200 OK`

Error bodies follow the `ApiErrorResponse` shape from `libs/api-macros`:
`{"error": <code>, "error_description": <description>}`. Tests assert on the
`error` code, not the human-readable description.

Because the database is fresh per run and per-test isolation is not provided,
each test generates its own unique email.

`e2e-tests/Dockerfile` builds the crate and runs `cargo test`. Build context is the
repo root so workspace manifests resolve. Cargo registry and target caching use
`RUN --mount=type=cache` to keep reruns fast.

### Lifecycle

Root `e2e.sh`:

```sh
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

P=identity-server-e2e
F=(-f identity-server/docker-compose.yml -f identity-server/docker-compose.e2e.yml)
E=(--env-file identity-server/.env --env-file identity-server/.env.test)
COMPOSE=(docker compose -p "$P" "${F[@]}" "${E[@]}")

"${COMPOSE[@]}" down -v --remove-orphans
trap '"${COMPOSE[@]}" down -v --remove-orphans' EXIT

"${COMPOSE[@]}" run --rm --build e2e
```

- **Isolation** comes from `-p identity-server-e2e`: its own containers, network,
  and volume namespace.
- **Cleanliness** comes from the pre-run `down -v`, which also recovers from a
  previous run that was hard-killed before its trap fired.
- **Ephemerality** comes from the `EXIT` trap. `down -v` removes the
  project-scoped `identity-server-db-data` volume.
- `run --rm` starts `db` → `migrate` → `identity-server` via `depends_on`, then
  propagates the test process's exit code as the script's exit code.

## Testing this design

The harness is verified by observation, not by tests of its own:

1. With the dev stack up (`docker compose up -d`), `./e2e.sh` completes green —
   proving isolation.
2. `docker ps -a`, `docker volume ls`, and `docker network ls` show no
   `identity-server-e2e` resources afterwards — proving cleanup.
3. Two consecutive runs both pass — proving a clean database each time.
4. Deliberately breaking an endpoint makes `e2e.sh` exit non-zero.
5. `docker compose -p identity-server-e2e ... exec` cannot reach the DB from the
   frontend network — proving the network boundary.

## Documentation

`DEPLOYMENT.md` is updated for: the new compose file layout, the network boundary,
the new env variables, `.env.test`, and an "End-to-end tests" section covering
`./e2e.sh`. Its "Adding a new service" checklist gains the override-file pattern.

## Risks

- **`internal: true` on the backend network** blocks all egress for the DB and
  migrate containers. Both use prebuilt images and need none. If a future service on
  that network needs outbound access, it must join the frontend network too.
- **`curl` in the runtime image** grows it slightly and adds a package to keep
  patched. Accepted for a working healthcheck.
- **Container build time** dominates the e2e loop. Cache mounts mitigate it; if it
  becomes painful, the fallback is publishing an ephemeral host port and running
  tests natively.
