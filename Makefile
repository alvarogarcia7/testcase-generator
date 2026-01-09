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

test: test-unit test-e2e
.PHONY: test

test-unit: build
	cargo test --all --all-features --tests
.PHONY: test-unit

test-doc:
	cargo test --doc
.PHONY: test-doc

clippy:
	cargo clippy --all --all-features --tests -- -D warnings
.PHONY: clippy

run: build
	./target/debug/testcase-manager
.PHONY: run

test-e2e-failing: build
	./tests/integration/run_e2e_test.sh
.PHONY: test-e2e-failing

test-e2e-failing-all: build
	./tests/integration/run_all_tests.sh
.PHONY: test-e2e-failing-all

test-e2e: test-e2e-validate-yaml
.PHONY: test-e2e

test-all: test test-e2e
.PHONY: test-all

test-e2e-validate-yaml: build
	cargo run --bin validate-yaml data/gsma_4.4.2.2_TC.yml data/schema.json >/dev/null 2>&1
	! cargo run --bin validate-yaml data/data.yml data/schema.json >/dev/null 2>&1
.PHONY: test-e2e-validate-yaml

