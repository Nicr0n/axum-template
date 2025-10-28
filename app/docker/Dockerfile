# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/reference/dockerfile/

################################################################################
# Create a stage for building the application.


ARG RUST_VERSION=1.90
ARG APP_NAME=axum-template-app
FROM rust:${RUST_VERSION} AS builder
ARG APP_NAME

WORKDIR /app
# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=app,target=app \
    --mount=type=bind,source=migration,target=migration \
    --mount=type=bind,source=infra,target=infra \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    set -e \
    && cargo build --locked --release \
    && cp ./target/release/app /bin/$APP_NAME

FROM debian:bookworm-slim

ARG APP_NAME=axum-template-app

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    axum-template-appuser
USER axum-template-appuser

WORKDIR /app

COPY --from=builder /bin/$APP_NAME /app/$APP_NAME

CMD ["/app/axum-template-app", "start"]
