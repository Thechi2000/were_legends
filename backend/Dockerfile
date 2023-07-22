FROM debian:bullseye AS runner
RUN apt-get update && apt-get upgrade -y && apt-get install -y ca-certificates


FROM rust:1.71 AS builder

RUN cargo install cargo-build-deps

RUN cargo new app
WORKDIR /app

COPY Cargo.toml ./

RUN cargo build-deps --release

COPY src src

RUN cargo build -r

FROM runner

WORKDIR /app
COPY --from=builder /app/target/release/backend app

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80

ENTRYPOINT [ "./app" ]
