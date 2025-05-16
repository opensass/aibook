# install cargo chef
FROM rust:1.86 AS chef
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    curl \
    git
RUN cargo install cargo-chef
RUN cargo install dioxus-cli
WORKDIR /app

# copy in source files, cd into target create and prepare recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN dx build --release
COPY . .

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt install -y openssl
RUN apt-get install ca-certificates
WORKDIR /app
COPY --from=builder /app/target/dx/aibook/release/web /usr/local/bin/web

# Make binary executable
RUN chmod +x /usr/local/bin/web/server

EXPOSE 80
EXPOSE 8080
EXPOSE 443

ENTRYPOINT ["/usr/local/bin/web/server"]
