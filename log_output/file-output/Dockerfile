FROM rust:latest AS builder
WORKDIR /usr/src/file-output
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/file-output /usr/local/bin/file-output
WORKDIR /usr/local/bin
CMD ["file-output"]
