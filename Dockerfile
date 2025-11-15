FROM rust:1.91.1-slim-bookworm AS builder

# Install musl tools for static linking
RUN apt-get update && apt-get install -y musl-tools

ADD . /app
WORKDIR /app

# Build for musl target
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static-debian12
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/matrix-ci-bot /bot
ENTRYPOINT [ "/bot" ]
