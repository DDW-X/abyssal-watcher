FROM rust:1.77 as builder
WORKDIR /app
COPY . .
RUN apt update && apt install -y pkg-config libssl-dev
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/abyssal_watcher /usr/local/bin/
CMD ["abyssal_watcher"]
