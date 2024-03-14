cargo +nightly fmt
cargo clippy --fix --allow-dirty --allow-staged -- -A clippy::needless_return
