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

To trigger the update, send a GET or POST request to:

```
http://localhost:8537/<compose-project>/<service-name>
```

I suggest using a reverse proxy to expose the webhook to the internet.
I use [caddy-docker-proxy](https://github.com/lucaslorentz/caddy-docker-proxy).

Unfortunately composehook can’t update itself as docker containers can’t restart themselves.
If you know a solution, please open an issue.
