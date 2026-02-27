pre-commit: test clippy coverage README_INSTALL_AUTOMATED.md
.PHONY: pre-commit

README_INSTALL_AUTOMATED.md:
	echo "" > README_INSTALL_AUTOMATED.md
	@for bin in $(shell cargo run --bin 2>&1| grep "^    "|awk '{print $1}'); do \
		echo "## $$bin " >> README_INSTALL_AUTOMATED.md ; \
		$$bin -- --help >> README_INSTALL_AUTOMATED.md; \
		echo "\`\`\`" >> README_INSTALL_AUTOMATED.md ; \
  		cargo run --bin $$bin -- --help >> README_INSTALL_AUTOMATED.md; \
		echo "\`\`\`" >> README_INSTALL_AUTOMATED.md ; \
  	done
.PHONY: README_INSTALL_AUTOMATED.md

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

build-script-cleanup:
	cargo build --bin script-cleanup
.PHONY: build-script-cleanup

run-script-cleanup: build-script-cleanup
	./target/debug/script-cleanup
.PHONY: run-script-cleanup

build-json-escape:
	cargo build --bin json-escape
.PHONY: build-json-escape

run-json-escape: build-json-escape
	./target/debug/json-escape
.PHONY: run-json-escape

test-e2e-failing: build
	./tests/integration/run_e2e_test.sh
	./tests/integration/test_variable_passing_e2e.sh

.PHONY: test-e2e-failing

test-e2e-failing-all: build
	./tests/integration/run_all_tests.sh
.PHONY: test-e2e-failing-all

test-e2e:
#	${MAKE} test-e2e-validate-yaml
#	${MAKE} test-e2e-orchestrator
#	${MAKE} test-e2e-orchestrator-examples
#	${MAKE} test-e2e-executor
#	#${MAKE} test-verify-sample
#	${MAKE} example_export-demo
	./tests/integration/check_environment.sh
	#./tests/integration/ci_test.sh
	#./tests/integration/run_all_tests.sh
	#./tests/integration/run_e2e_test.sh
	#./tests/integration/run_validate_files_test.sh
	./tests/integration/smoke_test.sh
	./tests/integration/test_bdd_e2e.sh
	#./tests/integration/test_bdd_initial_conditions.sh
	./tests/integration/test_executor_e2e.sh
	./tests/integration/test_manual_steps_e2e.sh
	./tests/integration/test_manual_verification_e2e.sh
	./tests/integration/test_orchestrator_e2e.sh
	./tests/integration/test_orchestrator_examples.sh
	#./tests/integration/test_run_manager_e2e.sh
	./tests/integration/test_validate_yaml_multi_e2e.sh
	./tests/integration/test_validate_yaml_watch_e2e.sh
	./tests/integration/test_variable_passing_e2e.sh
	#./tests/integration/test_verify_e2e.sh
.PHONY: test-e2e

example_export-demo:
	./examples/export_demo.sh
.PHONY: example_export-demo

test-all: test test-e2e
.PHONY: test-all

# Coverage exclusion pattern - escapes dots for regex
COVERAGE_EXCLUDE_REGEX = (fuzzy\\.rs|prompts\\.rs|main_editor\\.rs)

coverage:
	cargo llvm-cov --all-features --workspace --tests --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)' --fail-under-lines 50
.PHONY: coverage

coverage-e2e: build
	cargo llvm-cov clean --workspace
	cargo llvm-cov --all-features --workspace --no-report --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)'
	${MAKE} test-e2e
	cargo llvm-cov report --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)' --fail-under-lines 70
.PHONY: coverage-e2e

coverage-html:
	cargo llvm-cov --all-features --workspace --tests --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)' --html --open
.PHONY: coverage-html

coverage-html-e2e:
	${MAKE} coverage-e2e
	cargo llvm-cov report --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)' --html --open
.PHONY: coverage-html-e2e

coverage-lcov:
	cargo llvm-cov --all-features --workspace --tests --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)' --lcov --output-path target/llvm-cov/lcov.info
.PHONY: coverage-lcov

coverage-report:
	cargo llvm-cov report --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)'
.PHONY: coverage-report

coverage-report-e2e: build
	cargo llvm-cov clean --workspace
	cargo llvm-cov --all-features --workspace --no-report --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)'
	${MAKE} test-e2e
	cargo llvm-cov report --ignore-filename-regex '$(COVERAGE_EXCLUDE_REGEX)'
.PHONY: coverage-report-e2e

coverage-clean:
	cargo llvm-cov clean --workspace
.PHONY: coverage-clean

install-coverage-tools:
	./scripts/install-coverage-tools.sh --local
.PHONY: install-coverage-tools

verify-scripts:
	@echo "Verifying shell script syntax..."
	@FAILED=0; \
	for script in $$(find scripts tests/integration -type f -name "*.sh" 2>/dev/null); do \
		echo "Checking: $$script"; \
		if bash -n "$$script" 2>&1; then \
			echo "  ✓ PASSED"; \
		else \
			echo "  ✗ FAILED"; \
			FAILED=1; \
		fi; \
	done; \
	if [ $$FAILED -eq 1 ]; then \
		echo "Some script syntax checks failed"; \
		exit 1; \
	else \
		echo "All shell scripts have valid syntax"; \
	fi
.PHONY: verify-scripts

test-e2e-validate-yaml: build
	cargo run --bin validate-yaml -- --schema data/schema.json tests/sample/gsma_4.4.2.2_TC.yml >/dev/null 2>&1
	! cargo run --bin validate-yaml -- --schema data/schema.json tests/sample/data.yml >/dev/null 2>&1
	./tests/integration/test_validate_yaml_multi_e2e.sh
	./tests/integration/test_validate_yaml_watch_e2e.sh
.PHONY: test-e2e-validate-yaml

docker-build:
	${MAKE} README_INSTALL_AUTOMATED.md
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
		if cargo run --bin validate-yaml -- --schema data/schema.json "$$file" >/dev/null 2>&1; then \
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

test-e2e-orchestrator-examples: build
	./tests/integration/test_orchestrator_examples.sh
.PHONY: test-e2e-orchestrator-examples

