FROM rust:1.99-bookworm AS builder
WORKDIR /usr/src/pmcli
RUN apt-get update \
    && apt-get install -y --no-install-recommends perl make \
    && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends git ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/pmcli/target/release/pmcli /usr/local/bin/pmcli
ENTRYPOINT ["pmcli"]
CMD ["--help"]
