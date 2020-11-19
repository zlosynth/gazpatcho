.PHONY: clean
all: format lint check doc test

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

.PHONY: doc
doc:
	cargo doc

.PHONY: test
test:
	cargo test
