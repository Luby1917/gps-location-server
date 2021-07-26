# Build
FROM rust:1.53 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY ./src src
RUN cargo build --release
RUN cargo install --path . --verbose

# Production
FROM debian:buster-slim
WORKDIR /app

RUN apt-get update -y && apt-get install libpq-dev -y && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/geo /app
CMD ["./geo"]
