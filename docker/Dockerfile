FROM rust:1-slim-trixie AS chef
ENV RUSTUP_TOOLCHAIN=stable
WORKDIR /app
RUN cargo install cargo-chef --locked

# Stage 1: plan
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: build
FROM chef AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential pkg-config ca-certificates \
    && rm -rf /var/lib/apt/lists/*
{%- if use_db %}

ENV SQLX_OFFLINE=true
{%- endif %}

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json

COPY . .
{%- if layout == "workspace" %}
RUN cargo build --release --locked -p {{project-name}} --bin {{project-name}}
{%- else %}
RUN cargo build --release --locked --bin {{project-name}}
{%- endif %}

# Stage 3: runtime
FROM debian:trixie-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --system app && useradd --system --gid app app \
    && chown -R app:app /app

COPY --from=builder /app/target/release/{{project-name}} /usr/local/bin/{{project-name}}

RUN ldd /usr/local/bin/{{project-name}}

ENV RUST_LOG=info

USER app
ENTRYPOINT ["{{project-name}}"]
