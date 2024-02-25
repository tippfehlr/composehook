FROM rust as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

FROM debian:stable-slim
RUN apt update && apt install --yes --no-install-recommends \
    curl \
    ca-certificates \
    gnupg \
    && install -m 0755 -d /etc/apt/keyrings \
    && curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg \
    && chmod a+r /etc/apt/keyrings/docker.gpg \
    && echo \
         "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian \
         "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
         tee /etc/apt/sources.list.d/docker.list > /dev/null \
    && apt update \
    && apt --yes --no-install-recommends install \
         docker-ce-cli \
         docker-compose-plugin \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/composehook /usr/local/bin/composehook

VOLUME /compose
# https://gist.github.com/tippfehlr/843c2d11f356d37495670b5803b714f5
# echo "docker-label-webhooks" | sha256sum | grep -o '[1-9]' | head -n 4 | tr -d '\n'
EXPOSE 9411
ENTRYPOINT ["/usr/local/bin/composehook"]

