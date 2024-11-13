# install cargo chef & dioxus cli
FROM rust:1.80 AS chef
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

# build stage
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN dx build --release
RUN ls dist -lh
COPY . .

# runtime stage
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt install -y openssl ca-certificates libssl-dev libstdc++6
WORKDIR /app

# Copy the `dist` directory and the built binary to /usr/local/bin/dist
COPY --from=builder /app/dist /usr/local/bin/dist
COPY --from=builder /app/target/release/aibook /usr/local/bin/dist/aibook

# Make binary executable
RUN chmod +x /usr/local/bin/dist/aibook

# Expose ports
EXPOSE 8080
EXPOSE 443
EXPOSE 80

# Set PORT
ENV PORT=8080
RUN ls /usr/local/bin/dist -lh

ENTRYPOINT ["/usr/local/bin/dist/aibook"]

