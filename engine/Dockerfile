# Build stage
FROM rust:1.85 AS builder

# Install cargo-watch for live reloading
RUN cargo install cargo-watch

# Set working directory
WORKDIR /app

# Copy workspace root files
COPY Cargo.toml Cargo.lock ./

# Copy Cargo.toml files for all crates in the workspace
COPY gateway/Cargo.toml gateway/
COPY engine/Cargo.toml engine/
COPY storage/Cargo.toml storage/

# Cache dependencies by building a dummy version of engine
RUN mkdir -p engine/src && echo "fn main() {}" > engine/src/main.rs && cargo build --package engine && rm -rf engine/src

# Copy engine’s source code and configuration
COPY engine/src engine/src/
COPY engine/configuration engine/configuration/

# Final stage: runtime image
FROM rust:1.85

RUN cargo install cargo-watch

# Set working directory
WORKDIR /app

# Copy all necessary files from the builder stage
COPY --from=builder /app /app

# Expose the port (adjust if your service uses a different port)
EXPOSE 8081

# Run the engine microservice with live reloading
CMD ["cargo", "watch", "-x", "run --package engine"]
