# syntax=docker/dockerfile:1

FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM rust:1-slim-bookworm AS migrator
RUN cargo install sqlx-cli --no-default-features --features postgres
WORKDIR /app
COPY migrations ./migrations
ENTRYPOINT ["sqlx"]

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/identity-server ./identity-server
ENTRYPOINT ["./identity-server"]
