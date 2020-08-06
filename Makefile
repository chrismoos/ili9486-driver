.PHONY: test lib examples

all: test lib examples

lib:
	cargo build --lib

examples:
	cargo build --examples --features "examples,stm32f1xx"

test:
	cargo test --lib --target x86_64-apple-darwin
