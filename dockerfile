
# Stage 1: Build the application
FROM rust:1-bookworm AS builder

WORKDIR /app

# Copy dependency manifests first
COPY Cargo.toml Cargo.lock ./

# Copy application source and migrations
COPY src ./src
COPY migrations ./migrations

# Compile the application
RUN cargo build --locked --release

# Stage 2: Runtime image
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install certificates for HTTPS connections
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=builder /app/target/release/vellmont_backend /usr/local/bin/vellmont_backend

USER 10001:10001

# Expose the Axum port
EXPOSE 3000

# Start the backend
CMD ["vellmont_backend"]
