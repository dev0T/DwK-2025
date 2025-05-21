# Actix Server

Default port if not provided: `8080`.

## Development

The recommended way to run the server with auto-reloading with changes is by using `bacon`.

```zsh
bacon run-long
```

## Building

From the project root, run the following:

```zsh
# Compile
cargo build --release

# Run
cargo run --release
```

If successful, there will be a log message stating that the server is listening on the URL and PORT provided.

## Docker Image

To build the server docker image and push it to docker hub simply run the following command from the project root:

```zsh
docker buildx build -t <user>/todo-app:<tag> . --push
```