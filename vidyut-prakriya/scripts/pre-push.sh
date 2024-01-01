cargo fmt
cargo nextest run --no-fail-fast --status-level=fail
make test_all -i
cargo test --doc --no-fail-fast
