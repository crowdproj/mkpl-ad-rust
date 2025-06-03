FROM rust:latest AS build

WORKDIR /app
COPY . /app

RUN apt-get update \
    && apt-get install -y \
    libssl-dev openssl \
    pkg-config \
    curl \
    openjdk-17-jre-headless \
    && rm -rf /var/lib/apt/lists/* \
    && cargo build --release

FROM debian:trixie-slim

WORKDIR /app
RUN apt-get update && \
    apt-get install -y \
    libssl-dev openssl \
    pkg-config \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/app-hyper /app/app

ENTRYPOINT ["/app/app"]
