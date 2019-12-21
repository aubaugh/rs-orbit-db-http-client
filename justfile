# Runs the cli
run: build
    ./target/debug/cli -s https://localhost:3000 create-db counter counter

# Builds the cli
build:
    cargo build --bin cli

# Checks that `cargo fmt` has been run and runs `cargo test`
pre-push:
	cargo fmt -- --check
	cargo test
