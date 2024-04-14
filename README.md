# RATH Stack playground
Demo application for the RATH stack (Rust, Axum, Turso Htmx).  

The app runs at https://rust-api-plus-htmx.fly.dev/. It can take up to 20 seconds to respond for the first request, because the Turso db and Fly.io app automaticly scale down to 0.

## Getting started
Setup:
```sh
mv .env.example .env
# Edit .env file

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
- [x] Make a script that extracts htmx and hyperscript
- [x] Setup caching of htmx and hyperscript scripts properly
- [x] Use the correct http status codes in login and register
- [x] Add inline email validaton https://hypermedia.systems/hypermedia-systems/#_debouncing_our_validation_requests
- [x] Refactor into separate files for each route
- [x] Improve code for render_html.rs
- [x] Setup build via GH actions
- [x] Migrate from SQLX/PostgreSQL to Libsql/Turso (sqlx doesn't support turso yet... such a shame)
- [ ] Rename to RATH stack, Rust Axum Turso Hhtmx

## Handy commands
You can execute `prepush.sh` to fix lint and format.

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

Fly commands
```sh
fly machine start
fly machine stop

fly deploy # Deploy using remote runner
sudo fly deploy --local-only # Deploy using local docker runner
```
