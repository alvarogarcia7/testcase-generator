pre-commit: test clippy
.PHONY: pre-commit

lint: fmt clippy
.PHONY: lint

fmt:
	cargo fmt
.PHONY: fmt

build:
	cargo build --all
.PHONY: build

test: build
	cargo test --all --all-features --tests
.PHONY: test

test-doc:
	cargo test --doc
.PHONY: test-doc

clippy:
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy

run: build
	./target/debug/testcase-manager
.PHONY: run

