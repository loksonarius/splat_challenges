# Compilation Image
FROM rustlang/rust:nightly AS builder
WORKDIR /usr/src/splat_challenges

## Required Packages
RUN apt-get update && apt-get install -y ca-certificates libsqlite3-dev

## Source code and config files
COPY Cargo* diesel.toml .env ./
COPY src ./src
COPY migrations ./migrations

## DB file generation
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN mkdir ./db && diesel database setup

## Server Compilation
RUN cargo build --release

# Server Image
FROM debian:stretch-slim
EXPOSE 8000
WORKDIR /root

## Required Packages
RUN apt-get update && apt-get install -y ca-certificates libsqlite3-dev

## Import Built Resources
COPY Rocket.toml .
COPY --from=builder /usr/src/splat_challenges/target/release/splat_challenges .
COPY --from=builder /usr/src/splat_challenges/db ./db

## Run Server
CMD ["./splat_challenges"]
