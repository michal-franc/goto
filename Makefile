.PHONY: install

build:
	cargo build --release 

install:
	mv ./target/release/goto /usr/local/bin

uninstall:
	rm /usr/local/bin/goto
