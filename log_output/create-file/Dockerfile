FROM rust:latest AS builder
WORKDIR /usr/src/create-file
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/create-file  /usr/local/bin/create-file
WORKDIR /usr/local/bin
CMD ["create-file"]
