# syntax=docker/dockerfile:1

# Runs the end-to-end suite from inside the compose network. Build context is this
# directory: this crate has its own Cargo.toml and is not a workspace member, so
# it is self-contained.
FROM rust:1-slim-bookworm
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

# Compile at build time so a run is just execution. Deliberately no --mount=type=cache
# here: cache mounts exist only during the build, so anything written to one would
# have to be recompiled at run time.
RUN cargo test --no-run

CMD ["cargo", "test", "--offline", "--", "--nocapture"]
