# Project

## Project Structure

The application consists of two applications, `todo-app` and `todo-backend`. Both applications are web-servers using the Actix framework.

All accept the PORT variable but have default PORT values already set as follows:

- todo-app:      `8080` (being set through config to `3000`)
- todo-backend:  `8085` (being set through config to `3003`)

## Development

The recommended way to run the servers with auto-reloading with changes is by using `bacon`.

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

Before building the `todo-backend` image, run:

```zsh
cargo sqlx prepare
```

This will make sure the database migrations and queries will be available even when there's no connection to the database.

To build the image and push it to docker hub simply run the following command:

```zsh
docker buildx build -t <user>/<app>:<tag> <path> --push
```

## K8s

After creating a cluster, make sure the k8s node has the `/tmp/kube` folder present by running:

`docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube`

Create the namespace for the services:

`kubectl create namespace project`

To run the services, run the following command on `the_project` folder:

```zsh
kubectl apply -Rf manifests
```
