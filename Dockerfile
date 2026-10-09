# Build stage
FROM rust:bookworm-slim as builder

WORKDIR /usr/src/app
COPY . .

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install OpenSSL for reqwest
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /usr/src/app/target/release/rust_engine /app/rust_engine

# Copy static frontend files
COPY index.html /app/

# Expose the server port
EXPOSE 3000

# Run the server
CMD ["./rust_engine"]
