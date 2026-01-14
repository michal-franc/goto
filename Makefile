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
	mv ./target/release/goto /usr/local/bin

uninstall:
	rm /usr/local/bin/goto
