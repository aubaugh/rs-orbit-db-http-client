Rust OrbitDB HTTP Client
========================

> A client for OrbitDB's REST api that uses Rust's [async-std](http://docs.rs/async-std).

The client is a library crate that includes a separate command line interface binary

## To build and run the cli:
```
cargo build --bin cli
./target/debug/cli --help
```
or install and use [just](https://github.com/casey/just).

## To test the client library
```
RUST_TEST_THREADS=1 cargo test --lib
```
or
```
just test
```
