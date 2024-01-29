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
COPY . .
RUN cargo sqlx database setup
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
ENV DATABASE_URL="sqlite://sqlite.db"

COPY --from=builder /app/target/release/rust-plus-htmx-playground /usr/local/bin
COPY --from=builder /app/sqlite.db /app/sqlite.db
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static
RUN chmod +x /usr/local/bin/rust-plus-htmx-playground

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/rust-plus-htmx-playground"]
