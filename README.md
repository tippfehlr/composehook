# composehook

Create update webhooks for containers in docker compose projects.
Configured with docker labels!

## Installation

With docker compose:

```yaml
version: '3'
services:
  composehook:
    image: ghcr.io/tippfehlr/composehook:latest
    ports:
      - 8537:8537
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - <your compose project directory>:/compose
    environment:
      # (optional, default 10) in seconds.
      # only one update per timeout is allowed.
      # GitHub Webhooks may send multiple requests.
      - TIMEOUT=10
    restart: always
```

*[why this port?](https://gist.github.com/tippfehlr/843c2d11f356d37495670b5803b714f5)*

## Usage

To enable the webhook, add the `composehook.update=true`
label to the container in the compose file:

```yaml
version: '3'
services:
  example-service:
    image: example-image:latest
    labels:
      composehook.update: true
```

To trigger the update, send a POST request to:

```
http://localhost:8537/<compose-project>/<service-name>
```

I suggest using a reverse proxy to expose the webhook to the internet.
I use [caddy-docker-proxy](https://github.com/lucaslorentz/caddy-docker-proxy).

## HTTP status codes

- 200 OK: Update successful
- 400 Bad Request: label `composehook.update` not found or not set to true
- 404 Not Found: compose project or service not found
- 409 Conflict: Update not allowed by timeout or already in progress
- 500 Internal Server Error: couldn’t execute commands. Please open an issue.

## Details

When receiving a request, composehook will:

1. Check if the request is allowed by the timeout.
2. Check if the service exists (`docker compose ps -q <service>`)
3. Check if the service has the `composehook.update` label (`docker inspect <container_id>`)
4. Pull the latest image (`docker compose pull <service>`)
5. Recreate the container (`docker compose up -d <service>`)

Unfortunately composehook can’t update itself as docker containers can’t restart themselves.
If you know a solution, please open an issue.
