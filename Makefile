.PHONY: check fmt lint test build clean

check: fmt lint test

fmt:
	cargo fmt --check

lint:
	cargo clippy -- -D warnings

test:
	cargo test

build:
	cargo build --release

clean:
	cargo clean
