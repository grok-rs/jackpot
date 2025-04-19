# syntax=docker/dockerfile:1

# Build argument to specify the service (gateway, engine, or storage)
ARG SERVICE

# Stage 1: Install cargo-chef
FROM rust:1.85 AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Stage 2: Prepare dependency recipe for the workspace
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY gateway/Cargo.toml gateway/
COPY engine/Cargo.toml engine/
COPY storage/Cargo.toml storage/
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: Build dependencies and the service binary
FROM chef AS builder
ARG SERVICE  # Redeclare ARG here to make it available in this stage
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --package ${SERVICE}
COPY gateway/src gateway/src/
COPY engine/src engine/src/
COPY storage/src storage/src/
COPY gateway/configuration gateway/configuration/
COPY engine/configuration engine/configuration/
COPY storage/configuration storage/configuration/
COPY Cargo.toml Cargo.lock ./
COPY gateway/Cargo.toml gateway/
COPY engine/Cargo.toml engine/
COPY storage/Cargo.toml storage/
RUN cargo build --release --package ${SERVICE}

# Production target: Minimal runtime image
FROM debian:bookworm-slim AS prod
ARG SERVICE  # Redeclare ARG here to make it available in this stage
WORKDIR /app
COPY --from=builder /app/target/release/gateway /usr/local/bin/gateway
EXPOSE 8080
CMD ["/usr/local/bin/gateway"]

# Development target: Image with Rust toolchain and live reloading
FROM rust:1.85 AS dev
RUN cargo install cargo-watch
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY gateway/Cargo.toml gateway/
COPY engine/Cargo.toml engine/
COPY storage/Cargo.toml storage/
COPY gateway/src gateway/src/
COPY engine/src engine/src/
COPY storage/src storage/src/
COPY gateway/configuration gateway/configuration/
COPY engine/configuration engine/configuration/
COPY storage/configuration storage/configuration/
# Use environment variable for runtime service selection
ENV SERVICE=${SERVICE}
CMD ["sh", "-c", "cargo watch -x 'run --package ${SERVICE}'"]
