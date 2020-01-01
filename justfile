# Runs the cli
run: build
    ./target/debug/cli -s https://localhost:3000 get-dbs

# Builds the cli
build:
    cargo build --bin cli

# Runs the tests on one thread
test:
	RUST_TEST_THREADS=1 cargo test --lib
