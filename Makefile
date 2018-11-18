.PHONY: build clean default doc release run test

build:
	cargo build

clean:
	rm -rf target/

default: test

doc: build
	cargo doc

release:
	cargo build --release

run: build
	cargo run

test: build
	cargo test
