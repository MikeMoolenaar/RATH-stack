# RATH stack demo
Demo application for the RATH stack ([Rust](https://www.rust-lang.org/), [Axum](https://docs.rs/axum/latest/axum/), [Turso](https://turso.tech/) and [HTMX](https://htmx.org/)).  
Also includes [Tailwind CSS](https://tailwindcss.com/) and [Minijinja](https://docs.rs/minijinja/latest/minijinja/) (Template engine for Rust).

App runs at https://rust-api-plus-htmx.fly.dev. It can take up to 20 seconds to respond for the first request, because the Turso db and Fly.io app automaticly scale down to 0.

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
