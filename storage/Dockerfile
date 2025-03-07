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

# Cache dependencies by building a dummy version of storage
RUN mkdir -p storage/src && echo "fn main() {}" > storage/src/main.rs && cargo build --package storage && rm -rf storage/src

# Copy storage’s source code and configuration
COPY storage/src storage/src/
COPY storage/configuration storage/configuration/

# Final stage: runtime image
FROM rust:1.85

RUN cargo install cargo-watch

# Set working directory
WORKDIR /app

# Copy all necessary files from the builder stage
COPY --from=builder /app /app

# Expose the port (adjust if your service uses a different port)
EXPOSE 8082

# Run the storage microservice with live reloading
CMD ["cargo", "watch", "-x", "run --package storage"]
