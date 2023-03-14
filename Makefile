.PHONY: build run run-binary build

build:
	cargo build

run:
	cargo run

run-binary:
	./target/debug/rust-bankid

build:
	cargo build --release 

test:
	cargo test