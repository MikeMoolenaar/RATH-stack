FROM lukemathwalker/cargo-chef:0.1.62-rust-1.74-bookworm AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo install sqlx-cli

# Build application
ENV SQLX_OFFLINE=true 
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/rust-plus-htmx-playground /usr/local/bin
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static
COPY --from=builder /app/.env.prod /app/.env
RUN chmod +x /usr/local/bin/rust-plus-htmx-playground

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/rust-plus-htmx-playground"]
