
FROM rust:1.77 as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
    cargo build --release --manifest-path entrypoint/Cargo.toml

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/abyssal-watcher /app/abyssal-watcher

CMD ["./abyssal-watcher"]
