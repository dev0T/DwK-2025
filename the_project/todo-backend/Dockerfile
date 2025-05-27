FROM rust:bookworm AS builder
WORKDIR /usr/src/todo-backend
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/todo-backend /usr/local/bin/todo-backend
WORKDIR /usr/local/bin
CMD ["todo-backend"]