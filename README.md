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

# Run tailwind in another window
cd static
npm run tailwind
```

## TODO
- [x] Use serde more often (like parsing dates)
- [x] sanitize HTML by default for all fields
- [x] Add rate limiting, because why not https://github.com/jacob-pro/actix-extensible-rate-limit
- [x] Use Tailwind
- [x] Use templating like Askama (like listing todos in a list), or maybe just format! idk yet
- [x] Move from Artix to Axum
- [x] Implement Askama templating
- [x] Cleanup API and use Clippy for linting
- [x] Add navbar
- [x] Switch to MiniJinja (and this https://stackoverflow.com/questions/39639264/django-highlight-current-page-in-navbar)
- [x] Add register page with validation
- [x] Add a login page with validation
- [x] Implement authentication with session cookie
- [x] Deploy with docker to fly.io https://github.com/fly-apps/hello-rust
~~- [ ] Use Turso~~ Sqlx doesn't support turso...
- [x] Make a script that extracts htmx and hyperscript
- [x] Setup caching of htmx and hyperscript scripts properly
- [ ] Use the correct http status codes in login and register
- [ ] Rename to RATH stack, Rust Actix Turso Hhtmx
- [ ] use https://github.com/wilsonzlin/minify-html in a middleware: https://docs.rs/axum/latest/axum/middleware/fn.from_fn.html


## Handy commands
You can execute `prepushsh` to fix lint and format.

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
