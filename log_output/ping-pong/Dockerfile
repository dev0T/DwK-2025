FROM rust:latest AS builder
WORKDIR /usr/src/ping-pong
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ping-pong /usr/local/bin/ping-pong
WORKDIR /usr/local/bin
CMD ["ping-pong"]