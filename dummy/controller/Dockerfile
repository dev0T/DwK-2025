FROM clux/muslrust:stable AS builder
COPY . .
RUN --mount=type=cache,target=/volume/target \
  --mount=type=cache,target=/root/.cargo/registry \
  cargo install --bin dummycontroller --path=.

FROM cgr.dev/chainguard/static
COPY --from=builder --chown=nonroot:nonroot /opt/cargo/bin/dummycontroller /app/
EXPOSE 8080
ENTRYPOINT ["/app/dummycontroller"]