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

# Cache dependencies by building a dummy version of gateway
RUN mkdir -p gateway/src && echo "fn main() {}" > gateway/src/main.rs && cargo build --package gateway && rm -rf gateway/src

# Copy gateway’s source code and configuration
COPY gateway/src gateway/src/
COPY gateway/configuration gateway/configuration/

# Final stage: runtime image
FROM rust:1.85

RUN cargo install cargo-watch

# Set working directory
WORKDIR /app

# Copy all necessary files from the builder stage
COPY --from=builder /app /app

# Expose the port (adjust if your service uses a different port)
EXPOSE 8080

# Run the gateway microservice with live reloading
CMD ["cargo", "watch", "-x", "run --package gateway"]
