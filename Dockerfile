FROM rustlang/rust:nightly AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
COPY ./src/migrations /app/migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/projects-showcase-api .

EXPOSE 8000

CMD ["./projects-showcase-api"]
