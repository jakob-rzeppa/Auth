# syntax=docker/dockerfile:1

FROM rust:1-slim-bookworm AS builder

# git is needed for the api-macros / fnmock git dependencies.
RUN apt-get update \
    && apt-get install -y --no-install-recommends git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock askama.toml ./
COPY src ./src
# Askama compiles the templates into the binary.
COPY templates ./templates

RUN cargo build --release

FROM rust:1-slim-bookworm AS migrator
RUN cargo install sqlx-cli --no-default-features --features postgres
WORKDIR /app
COPY migrations ./migrations
ENTRYPOINT ["sqlx"]

FROM debian:bookworm-slim AS runtime
# curl is required by the compose healthcheck; the slim image ships neither
# curl nor wget.
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/auth-server ./auth-server
# Read at runtime (persistence/clients/store.rs).
COPY seed ./seed
ENTRYPOINT ["./auth-server"]
