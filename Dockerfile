# Use Rust official image for building
FROM rustlang/rust:nightly AS builder

# Set the working directory
WORKDIR /app

# Copy dependencies first for better caching
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
COPY ./src/migrations /app/migrations

# Build the application
RUN cargo build --release

# Use a smaller base image for the final app
FROM debian:bookworm-slim

# Install dependencies for Rocket and PostgreSQL client
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/projects-showcase-api .

# Expose the Rocket port
EXPOSE 8000

# Start the application
CMD ["./projects-showcase-api"]
