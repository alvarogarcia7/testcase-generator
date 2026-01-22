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

run-trm: build
	./target/debug/trm
.PHONY: run-trm

run-test-verify: build
	./target/debug/test-verify
.PHONY: run-test-verify

test-e2e-failing: build
	./tests/integration/run_e2e_test.sh
.PHONY: test-e2e-failing

test-e2e-failing-all: build
	./tests/integration/run_all_tests.sh
.PHONY: test-e2e-failing-all

test-e2e:
	${MAKE} test-e2e-validate-yaml
	${MAKE} test-e2e-orchestrator
	#${MAKE} test-e2e-executor
	#${MAKE} test-verify-sample
	${MAKE} example_export-demo
.PHONY: test-e2e

example_export-demo:
	./examples/export_demo.sh
.PHONY: example_export-demo

test-all: test test-e2e
.PHONY: test-all

test-e2e-validate-yaml: build
	cargo run --bin validate-yaml tests/sample/gsma_4.4.2.2_TC.yml data/schema.json >/dev/null 2>&1
	! cargo run --bin validate-yaml tests/sample/data.yml data/schema.json >/dev/null 2>&1
.PHONY: test-e2e-validate-yaml

test-verify-sample: build
	./tests/integration/test_verify_e2e.sh
.PHONY: test-verify-sample

validate-all-testcases: build
	SCHEMA_FILE=data/schema.json ./scripts/validate-files.sh --pattern '\.ya?ml$$' --validator ./scripts/validate-yaml-wrapper.sh
.PHONY: validate-all-testcases

watch: build
	./scripts/watch-yaml-files.sh
.PHONY: watch

watch-verbose: build
	SCHEMA_FILE=data/schema.json ./scripts/validate-files.sh --pattern '\.ya?ml$$' --validator ./scripts/validate-yaml-wrapper.sh --watch --verbose
.PHONY: watch-verbose

clean-validation-cache:
	rm -rf .validation-cache/
.PHONY: clean-validation-cache

run-test-executor: build
	cargo run --bin test-executor
.PHONY: run-test-executor

test-executor-sample: build
	@echo "Testing test-executor against sample test cases..."
	@echo "Generating script from gsma_4.4.2.2_TC.yml..."
	cargo run --bin test-executor -- generate tests/sample/gsma_4.4.2.2_TC.yml >/dev/null
	@echo "✓ Script generation verified for gsma_4.4.2.2_TC.yml"
	@echo "Generating script from SGP.22_4.4.2.yaml..."
	cargo run --bin test-executor -- generate tests/sample/SGP.22_4.4.2.yaml >/dev/null
	@echo "✓ Script generation verified for SGP.22_4.4.2.yaml"
	@echo "All test-executor sample verifications passed!"
.PHONY: test-executor-sample

test-e2e-executor: build
	./tests/integration/test_executor_e2e.sh
.PHONY: test-e2e-executor

test-e2e-orchestrator: build
	./tests/integration/test_orchestrator_e2e.sh
	cargo run --bin test-orchestrator run testcases/self_validated_example.yml --verbose
.PHONY: test-e2e-orchestrator

