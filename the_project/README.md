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

## DBaaS vs DIY

### DBaaS (Google Cloud SQL)

#### Pros

- High level of abstraction makes it fast and easy to setup and scalate.
- Less maintenance is needed for being fully managed, allowing resources to be used elsewere.
- Services are usually stable and guaranteed uptime.
- Support offered by the platform in case of any issues.
- Backups are automated, with configurable options.

#### Cons

- Not fully configurable as a DIY instance, configuration options might be disabled by the vendor due to limitations.
- Backups take storage space and might be charged accordinly.
- Sensitive data with specific requirements might not be guaranteed by the platform.
- Migrations can be difficult.
- More specific use cases might be difficult or expensive.

### DIY

#### Pros

- Highly configurable, with full root access.
- Backup and logging can be configured to multiple services which won't rely on the same platform and can be accessed even if there's an outage.
- Location of deployment and hosting isn't limited. 
- Can make usage of free and open-source tooling.
- Easy to migrate.

#### Cons

- Need manual configuration for everything, including backups and logging.
- Knowledge is required to garantee availability and reliability which might incur in labour costs.
- Restoring backups might be tricky if manual intervention is needed, often relying in downtime.