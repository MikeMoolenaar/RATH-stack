# Actix web test

## Getting started
Setup:
```sh
cargo install sqlx-cli
cargo sqlx database setup
```

Run:
```sh
cargo watch -c -x run -i src/static/**
```

## TODO
- [ ] Use serde more often (like parsing dates) + sanitize HTML by default for all fields
- [ ] Use Tailwind
- [ ] Use templating like Askama (like listing todos in a list), or maybe just format! idk yet
- [ ] Rename to RATH stack, Rust Actix Turso Hhtmx
- [ ] Deploy with docker to fly.io https://github.com/fly-apps/hello-rust
- [ ] Use Turso
