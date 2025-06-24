.PHONY: build test run-example fmt lint clean install

build:
	cargo build --release

test:
	cargo test

run-example:
	cargo run -- examples/work_day.zbx

fmt:
	cargo fmt

lint:
	cargo clippy

clean:
	cargo clean

install:
	cargo install --path .

gen-docs:
	cargo doc --no-deps

all: fmt lint test build