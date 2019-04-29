FROM rustlang/rust:nightly AS builder

WORKDIR /usr/src/splat_challenges
COPY Cargo* Rocket.toml diesel.toml ./
COPY src ./src
COPY migrations ./migrations
RUN ls -R
RUN cargo build

FROM alpine:latest
RUN apk --no-cache add ca-certificates

WORKDIR /root
COPY --from=builder /usr/src/splat_challenges/splat_challenges .

CMD ["./splat_challenges"]

