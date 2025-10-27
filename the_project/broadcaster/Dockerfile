FROM rust:latest AS builder
WORKDIR /usr/src/broadcaster
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/broadcaster  /usr/local/bin/broadcaster
WORKDIR /usr/local/bin
CMD ["broadcaster"]