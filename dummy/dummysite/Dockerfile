FROM rust:latest AS builder
WORKDIR /usr/src/dummysite
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/dummysite  /usr/local/bin/dummysite
WORKDIR /usr/local/bin
CMD ["dummysite"]