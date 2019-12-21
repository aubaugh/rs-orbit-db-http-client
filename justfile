# Runs the cli
run: build
    ./target/debug/cli -s https://localhost:3000 create-db counter counter

# Builds the cli
build:
    cargo build --bin cli

# Formats the source code and runs tests
pre-push:
	cargo fmt
	cargo test --lib
