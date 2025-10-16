# Rust build stage
FROM rust:1.74 as builder
WORKDIR /usr/src/bolt
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/bolt/target/release/bolt /usr/local/bin/bolt
# default config path
VOLUME ["/etc/circleos", "/opt/circleos/plugins"]
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/bolt"]
CMD ["--config", "/etc/circleos/config.toml"]
