# ---------------------------------------------------------------------------
# migrator: one-shot image that applies the migrations and exits.
# ---------------------------------------------------------------------------
FROM rust:1-slim-trixie AS migrator

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install sqlx-cli --version ^0.9 --locked --no-default-features --features postgres,rustls

WORKDIR /app
COPY migrations ./migrations

# Reads DATABASE_URL from the environment.
ENTRYPOINT ["sqlx"]
CMD ["migrate", "run", "--source", "migrations"]

# ---------------------------------------------------------------------------
# dev: rebuilds and restarts the server on every source change.
# ---------------------------------------------------------------------------
FROM rust:1-slim-trixie AS dev

# git for the api-macros and fnmock git dependencies, curl for the compose
# healthcheck.
RUN apt-get update \
    && apt-get install -y --no-install-recommends git ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo install cargo-watch --locked

WORKDIR /app

COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock sqlx.toml askama.toml ./
COPY .sqlx ./.sqlx
COPY migrations ./migrations
COPY seed ./seed
COPY templates ./templates
COPY src ./src

CMD ["cargo", "watch", "-x", "run"]

# ---------------------------------------------------------------------------
# builder: compiles the auth-server binary.
#
# The sqlx query macros are checked offline against the metadata in .sqlx (see
# .cargo/config.toml), since no database is reachable during the build.
# Regenerate it with `cargo sqlx prepare` after changing a query or migration,
# see DEV.md.
# The askama templates are compiled into the binary.
# ---------------------------------------------------------------------------
FROM rust:1-slim-trixie AS builder

# git is needed for the api-macros and fnmock git dependencies.
RUN apt-get update \
    && apt-get install -y --no-install-recommends git ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock sqlx.toml askama.toml ./
COPY .sqlx ./.sqlx
COPY templates ./templates
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --locked \
    && cp target/release/auth-server /usr/local/bin/auth-server

# ---------------------------------------------------------------------------
# runtime: the server itself. Does not run migrations.
# ---------------------------------------------------------------------------
FROM debian:trixie-slim AS runtime

# curl is used by the compose healthcheck.
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --system --no-create-home auth-server

WORKDIR /app
COPY --from=builder /usr/local/bin/auth-server ./auth-server
# Read at runtime via the relative path seed/clients.yaml.
COPY seed ./seed

USER auth-server
ENTRYPOINT ["./auth-server"]
