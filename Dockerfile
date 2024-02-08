FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# RUN cargo install sqlx-cli


# Build application
ENV SQLX_OFFLINE=true 
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.19 AS runtime
WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-plus-htmx-playground /usr/local/bin
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/static /app/static
# Optionally copy the .env file
COPY --from=builder /app/.env.prod* /app/.env
RUN chmod +x /usr/local/bin/rust-plus-htmx-playground

EXPOSE 8080
ENTRYPOINT ["rust-plus-htmx-playground"]
