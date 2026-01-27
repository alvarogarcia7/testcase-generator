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

test:
	${MAKE} test-unit
	${MAKE} test-e2e
	${MAKE} test-tagged-example
	#${MAKE} verify-testcases
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

docker-build:
	docker build -t testcase-manager:latest .
.PHONY: docker-build

docker-run:
	docker run -v $(PWD)/testcases:/app/testcases testcase-manager:latest
.PHONY: docker-run

test-verify-sample: build
	./tests/integration/test_verify_e2e.sh
.PHONY: test-verify-sample

validate-all-testcases: build
	SCHEMA_FILE=data/schema.json ./scripts/validate-files.sh --pattern '\.ya?ml$$' --validator ./scripts/validate-yaml-wrapper.sh
.PHONY: validate-all-testcases

verify-testcases: build
	@echo "Verifying test case files against schema..."
	@FAILED=0; \
	for file in $$(find testcases tests/sample data -type f \( -name "*.yml" -o -name "*.yaml" \) -not \( -name "*te.y*" -o -iname "sample_test_runs.yaml" -o -name "*wrong*" \) 2>/dev/null); do \
		echo "Validating: $$file"; \
		if cargo run --bin validate-yaml "$$file" data/schema.json >/dev/null 2>&1; then \
			echo "  ✓ PASSED"; \
		else \
			echo "  ✗ FAILED"; \
			FAILED=1; \
		fi; \
	done; \
	if [ $$FAILED -eq 1 ]; then \
		echo "Some validations failed"; \
		exit 1; \
	else \
		echo "All test case files validated successfully"; \
	fi
.PHONY: verify-testcases

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
	! cargo run --bin test-orchestrator run testcases/self_validated_example_wrong.yml
.PHONY: test-e2e-orchestrator

test-tagged-example: build
	@echo "Testing tagged test case example..."
	@echo "Validating example_tagged_test.yml against schema..."
	cargo run --bin validate-yaml testcases/example_tagged_test.yml data/schema.json >/dev/null 2>&1
	@echo "✓ Schema validation passed"
	@echo "Testing tag listing functionality..."
	cargo run --bin test-orchestrator show-tags TC_TAGGED_EXAMPLE >/dev/null 2>&1
	@echo "✓ Tag listing passed"
	@echo "Testing tag filtering with include tags..."
	cargo run --bin test-orchestrator find-by-tag smoke >/dev/null 2>&1
	@echo "✓ Tag filtering passed"
	@echo "All tagged test case example verifications passed!"
.PHONY: test-tagged-example

test-filter-smoke: build
	@echo "Running smoke tests..."
	cargo run --bin test-orchestrator run-all --include-tags smoke
.PHONY: test-filter-smoke

test-filter-fast: build
	@echo "Running fast tests..."
	cargo run --bin test-orchestrator run-all --include-tags fast
.PHONY: test-filter-fast

test-filter-priority-high: build
	@echo "Running priority-high tests..."
	cargo run --bin test-orchestrator run-all --include-tags priority-high
.PHONY: test-filter-priority-high

test-filter-automated: build
	@echo "Running automated-only tests..."
	cargo run --bin test-orchestrator run-all --dynamic-tags --include-tags automated-only
.PHONY: test-filter-automated

test-filter-no-slow: build
	@echo "Running tests excluding slow tests..."
	cargo run --bin test-orchestrator run-all --exclude-tags slow
.PHONY: test-filter-no-slow

test-filter-expression: build
	@echo "Running tests with complex expression..."
	cargo run --bin test-orchestrator run-all --tag-expr "(smoke || regression) && !slow"
.PHONY: test-filter-expression

test-filter-all: build
	@echo "Testing all tag filter capabilities..."
	${MAKE} test-filter-smoke
	${MAKE} test-filter-fast
	${MAKE} test-filter-priority-high
	${MAKE} test-filter-automated
	${MAKE} test-filter-no-slow
	${MAKE} test-filter-expression
	@echo "All tag filter tests passed!"
.PHONY: test-filter-all

