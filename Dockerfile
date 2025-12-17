FROM rust:1.92.0-slim-trixie AS builder

WORKDIR /usr/src/

COPY . .

# Build Deps
RUN apt update
RUN apt-get install -y pkgconf libssl-dev libc6

# Rebuild front end in release mode
RUN cargo install dioxus-cli --locked
RUN cd frontend/packages/web && dx build --release && cd ../../../
RUN cargo build --release

FROM debian:trixie-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/target/dx/dioxus_web/release/web/public/ target/dx/dioxus_web/release/web/public/
COPY --from=builder /usr/src/frontend/fallback.html frontend/fallback.html
COPY --from=builder /usr/src/config config
COPY --from=builder /usr/src/target/release/loco_test-cli loco_test-cli

ENV DIOXUS_PUBLIC_PATH="./target/dx/dioxus_web/release/web/public"

ENTRYPOINT ["/usr/app/loco_test-cli", "-e", "production", "start"]
