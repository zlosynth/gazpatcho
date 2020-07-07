.PHONY: clean
all: format lint check test

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy

.PHONY: check
check:
	cargo check
	cargo check --examples

.PHONY: test
test:
	cargo test
