ARG RUST_VERSION=1.95

FROM lukemathwalker/cargo-chef:latest-rust-${RUST_VERSION}-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin as2mca-mock

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --system --gid 10001 as2mca \
    && useradd --system --uid 10001 --gid as2mca --shell /usr/sbin/nologin as2mca

WORKDIR /app

RUN chown -R as2mca:as2mca /app
RUN mkdir -p /data \
    && chown -R as2mca:as2mca /data

COPY --from=builder --chown=as2mca:as2mca /app/target/release/as2mca-mock ./as2mca-mock

ENV AS2MCA_MOCK_CACHE_PATH=/data/cache.db

LABEL maintainer="fe.offep@gmail.com" \
      org.opencontainers.image.title="as2mca-mock" \
      org.opencontainers.image.source="https://github.com/plp-lang/as2mca-mock" \
      org.opencontainers.image.description="Aplication Server 2MCA API HTTP Mock Server" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.authors="Falldot, fe.offep@gmail.com"

USER as2mca
EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
    CMD curl -fsS http://localhost:3000/health || exit 1

ENTRYPOINT ["./as2mca-mock"]
