FROM rust:1.91.1-slim-bookworm as builder
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/static-debian12
COPY --from=builder /app/target/release/matrix-ci-bot /bot
ENTRYPOINT [ "/bot" ]
