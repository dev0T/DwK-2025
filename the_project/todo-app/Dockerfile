FROM rust:latest AS builder
WORKDIR /usr/src/todo-app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/todo-app /usr/local/bin/todo-app
COPY ./static/. /usr/local/bin/static/
WORKDIR /usr/local/bin
CMD ["todo-app"]