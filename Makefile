.PHONY: build test lint install uninstall

build: lint test
	cargo build --release

test:
	cargo test

lint:
	cargo clippy -- -D warnings
	cargo fmt --check

fmt:
	cargo fmt

install:
	cargo install --path .

uninstall:
	cargo uninstall goto 2>/dev/null || true
