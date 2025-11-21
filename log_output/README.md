# Log Output

## Project Structure

The application consists of three applications, `create-file`, `file-output` and `ping-pong`. All three applications are web-servers using the Actix framework.

All three applications accept the PORT variable but have default PORT values already set as follows:

- create-file:  `8083`
- file-output:  `8080`
- ping-pong:    `8080` (later being set to `3006`)

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

To build the server docker image and push it to docker hub simply run the following command from the project root:

```zsh
docker buildx build -t <user>/<app>:<tag> <path> --push
```

## K8s

After creating a cluster, make sure the k8s node has the `/tmp/kube` folder present by running:

`docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube`

Create the namespace for the services:

`kubectl create namespace exercises`

To run the services, run the following command on log-output folder:

```zsh
kubectl apply -k .
```

## Continuous Deployment (ArgoCD)