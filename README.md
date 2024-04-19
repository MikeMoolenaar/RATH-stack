# RATH stack demo
Demo application for the RATH stack (Rust + Axum + Turso + HTMX).  

App runs at https://rust-api-plus-htmx.fly.dev. It can take up to 20 seconds to respond for the first request, because the Turso db and Fly.io app automaticly scale down to 0.

## Technoligies
- [Axum](https://docs.rs/axum/latest/axum/) - web API framework for Rust
- [Turso](https://turso.tech) - Sqlite based database
- [HTMX](https://htmx.org) - HTML-first web library
- [Tailwind CSS](https://tailwindcss.com/) + [daisyui](https://daisyui.com/) - class based CSS
- [Minijinja](https://docs.rs/minijinja/latest/minijinja) - templating engine for Rust
- [Fly.io](https://fly.io/) - hosting in Docker containers

## Getting started
First, create a [Turso](https://turso.tech/) account and get the DB url + Auth token.  

Setting up:
```sh
mv .env.example .env
# Edit .env file and set the correct values

npm install --prefix src/static
npm install -g tailwindcss # Or install tailwind via your package manager
```

Running the app:
```sh
cargo run
# Or run when any non-static file (-i) changes and clear console (-c)
cargo watch -c -x run -i /static

# Run tailwind in another window
cd static
npm run tailwind
```

## Handy commands
You can execute `prepush.sh` to fix lint and format.

Run lint
```sh
# Check
cargo clippy -- -A clippy::needless_return
# Fix
cargo clippy --allow-dirty --fix -- -A clippy::needless_return
```

Run formatter
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
