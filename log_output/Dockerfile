FROM rust:latest as builder
WORKDIR /usr/src/log-output
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/log-output /usr/local/bin/log-output
CMD ["log-output"]