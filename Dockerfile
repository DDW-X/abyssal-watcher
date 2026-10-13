
FROM rust:1.77 as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev && \
    cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/abyssal_watcher /app/abyssal_watcher

CMD ["./abyssal_watcher"]
