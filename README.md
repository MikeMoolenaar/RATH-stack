# Rust + HTMX playground

## Getting started
Setup:
```sh
cargo install sqlx-cli
cargo sqlx database setup

npm install --prefix src/static
npm install -g tailwindcss # Or install tailwind via your package manager
```

Run:
```sh
cargo run
# Or rerun when any non-static file changes (-x) and clear console (-c)
cargo watch -c -x run -i /static

tailwindcss -c static/tailwind.config.js -o static/output.css --watch
```

## TODO
- [x] Use serde more often (like parsing dates)
- [x] sanitize HTML by default for all fields
- [x] Add rate limiting, because why not https://github.com/jacob-pro/actix-extensible-rate-limit
- [x] Use Tailwind
- [x] Use templating like Askama (like listing todos in a list), or maybe just format! idk yet
- [x] Move from Artix to Axum
- [x] Implement Askama templating
- [ ] Cleanup API and use Clippy for linting
- [x] Add navbar
- [ ] Add register page with validation
- [ ] Add a lgoin page with validation
- [ ] Deploy with docker to fly.io https://github.com/fly-apps/hello-rust  
~~- [ ] Use Turso~~ Sqlx doesn't support turso...
- [ ] Rename to RATH stack, Rust Actix Turso Hhtmx


## Handy commands
run lint
```sh
# Check
cargo clippy -- -A clippy::needless_return
# Fix
cargo clippy --allow-dirty --fix -- -A clippy::needless_return
```

run formatter
```sh
cargo +nightly fmt -- src/routes.rs
```
