pre-commit: build lint test
.PHONY: pre-commit

README_INSTALL_AUTOMATED.md:
	echo "" > README_INSTALL_AUTOMATED.md
	@for bin in $(shell cargo run --bin 2>&1| grep "^    "|awk '{print $1}'); do \
		cargo build --bin $$bin ; \
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
	cargo fmt --all
.PHONY: fmt

build:
	cargo build --workspace
.PHONY: build

build-debug:
	cargo build --workspace
.PHONY: build-debug

build-release:
	cargo build --workspace --release
.PHONY: build-release

# Per-crate build targets
build-validate-yaml:
	cargo build --package validate-yaml
.PHONY: build-validate-yaml

build-validate-json:
	cargo build --package validate-json
.PHONY: build-validate-json

build-verifier:
	cargo build --package verifier
.PHONY: build-verifier

build-test-executor:
	cargo build --package test-executor
.PHONY: build-test-executor

build-test-orchestrator:
	cargo build --package test-orchestrator
.PHONY: build-test-orchestrator

build-test-run-manager:
	cargo build --package test-run-manager
.PHONY: build-test-run-manager

build-test-verify:
	cargo build --package test-verify
.PHONY: build-test-verify

build-script-cleanup:
	cargo build --package script-cleanup
.PHONY: build-script-cleanup

build-json-escape:
	cargo build --package json-escape
.PHONY: build-json-escape

build-json-to-yaml:
	cargo build --package json-to-yaml
.PHONY: build-json-to-yaml

build-editor:
	cargo build --package editor
.PHONY: build-editor

build-testcase-manager:
	cargo build --package testcase-manager
.PHONY: build-testcase-manager

build-tpdg-compat:
	cargo build --package tpdg-compat
.PHONY: build-tpdg-compat

build-bash-eval:
	cargo build --package bash-eval
.PHONY: build-bash-eval

setup-python-for-test:
	@if command -v uv > /dev/null 2>&1; then \
		echo "Setting up Python environment for tests..."; \
		uv sync > /dev/null 2>&1 || true; \
		uv python install 3.14 > /dev/null 2>&1 || true; \
		echo "✓ Python environment ready"; \
	else \
		echo "Warning: uv not installed, skipping Python setup"; \
	fi
.PHONY: setup-python-for-test

test: setup-python-for-test
	${MAKE} test-unit
	${MAKE} test-e2e
	${MAKE} verify-testcases
# 	${MAKE} generate-docs-coverage
	${MAKE} coverage-clean
.PHONY: test

test-unit: build
	cargo test --workspace --all-features --tests
.PHONY: test-unit

test-doc:
	cargo test --workspace --doc
.PHONY: test-doc

clippy:
	cargo clippy --workspace --all-features --tests -- -D warnings
.PHONY: clippy

run: build
	./target/debug/testcase-manager
.PHONY: run

run-trm: build-test-run-manager
	./target/debug/trm
.PHONY: run-trm

run-test-verify: build-test-verify
	./target/debug/test-verify
.PHONY: run-test-verify

run-script-cleanup: build-script-cleanup
	./target/debug/script-cleanup
.PHONY: run-script-cleanup

run-json-escape: build-json-escape
	./target/debug/json-escape
.PHONY: run-json-escape

run-verifier: build-verifier
	./target/debug/verifier
.PHONY: run-verifier

test-e2e-verifier-container: build
	./tests/integration/test_verifier_container_e2e.sh
.PHONY: test-e2e-verifier-container

test-verifier-edge-cases: build
	cargo test verification_edge_cases_test
#	./tests/integration/test_verifier_edge_cases_e2e.sh
.PHONY: test-verifier-edge-cases

test-e2e-failing: build
	./tests/integration/run_e2e_test.sh
	./tests/integration/test_variable_passing_e2e.sh

.PHONY: test-e2e-failing

test-e2e-failing-all: build
	./tests/integration/run_all_tests.sh
.PHONY: test-e2e-failing-all

test-e2e: build
#	${MAKE} test-e2e-validate-yaml
#	${MAKE} test-e2e-orchestrator
#	${MAKE} test-e2e-orchestrator-examples
#	${MAKE} test-e2e-executor
#	#${MAKE} test-verify-sample
#	${MAKE} example_export-demo
	./crates/testcase-manager/tests/integration/check_environment.sh
#	./crates/testcase-manager/tests/integration/ci_test.sh
#	./crates/testcase-manager/tests/integration/run_all_tests.sh
#	./crates/testcase-manager/tests/integration/run_e2e_test.sh
#	#./crates/testcase-manager/tests/integration/run_validate_files_test.sh
	./crates/testcase-manager/tests/integration/smoke_test.sh
#	./crates/testcase-manager/tests/integration/test_bdd_e2e.sh
#	#./crates/testcase-manager/tests/integration/test_bdd_initial_conditions.sh
	./crates/testcase-manager/tests/integration/test_executor_e2e.sh
#	./crates/testcase-manager/tests/integration/test_manual_steps_e2e.sh
#	./crates/testcase-manager/tests/integration/test_manual_verification_e2e.sh
	./crates/testcase-manager/tests/integration/test_orchestrator_e2e.sh
#	./crates/testcase-manager/tests/integration/test_orchestrator_examples.sh
#	#./crates/testcase-manager/tests/integration/test_run_manager_e2e.sh
	./crates/testcase-manager/tests/integration/test_validate_yaml_watch_e2e.sh
#	./crates/testcase-manager/tests/integration/test_validate_yaml_multi_e2e.sh
#	./crates/testcase-manager/tests/integration/test_validate_yaml_schema_watch_e2e.sh
#	./crates/testcase-manager/tests/integration/test_validate_yaml_transitive_schema_watch_e2e.sh
#	./crates/testcase-manager/tests/integration/test_auto_schema_validation_e2e.sh
	./crates/testcase-manager/tests/integration/test_variable_passing_e2e.sh
	./crates/testcase-manager/tests/integration/test_verifier_e2e.sh
#	./crates/testcase-manager/tests/integration/test_verifier_container_e2e.sh
	${MAKE} test-verifier-edge-cases
#	./crates/testcase-manager/tests/integration/test_verify_e2e.sh
#	./crates/testcase-manager/tests/integration/test_container_yaml_compat_e2e.sh
#	./crates/testcase-manager/tests/integration/test_documentation_generation.sh
	# Valid values of BUILD_VARIANT are "" (debug) or "--release" (release mode)
#	BUILD_VARIANT="" ./scripts/run_verifier_and_generate_reports.sh
	${MAKE} validate-output-schemas
.PHONY: test-e2e

example_export-demo:
	./examples/export_demo.sh
.PHONY: example_export-demo

test-all: test test-e2e
.PHONY: test-all

# Coverage exclusion pattern - escapes dots for regex
COVERAGE_EXCLUDE_REGEX = (crates/testcase-manager/src/fuzzy\\.rs|crates/testcase-manager/src/prompts\\.rs|crates/testcase-manager/src/main_editor\\.rs)

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

clean:
	${MAKE} coverage-clean
	@if command -v sccache >/dev/null 2>&1; then \
		${MAKE} sccache-clean; \
	fi
.PHONY: clean

coverage-clean:
	@-cargo llvm-cov clean --workspace > /dev/null 2>&1 || true
	rm -f ./*.profraw
.PHONY: coverage-clean

install-coverage-tools:
	./scripts/install-coverage-tools.sh --local
.PHONY: install-coverage-tools

install-sccache:
	./scripts/install-sccache.sh --local
.PHONY: install-sccache

enable-sccache:
	@echo "To enable sccache, run:"
	@echo "  source ./scripts/enable-sccache.sh"
	@echo ""
	@echo "Or for permanent setup:"
	@echo "  source ./scripts/enable-sccache.sh --permanent"
	@echo ""
	@echo "To check if sccache is enabled:"
	@echo "  source ./scripts/enable-sccache.sh --check"
.PHONY: enable-sccache

disable-sccache:
	@echo "To disable sccache, run:"
	@echo "  source ./scripts/disable-sccache.sh"
	@echo ""
	@echo "Or manually:"
	@echo "  unset RUSTC_WRAPPER"
.PHONY: disable-sccache

sccache-stats:
	@sccache --show-stats
.PHONY: sccache-stats

sccache-check:
	@if [ -z "$$RUSTC_WRAPPER" ]; then \
		echo "❌ sccache is NOT enabled"; \
		echo ""; \
		echo "To enable sccache:"; \
		echo "  source ./scripts/enable-sccache.sh"; \
		echo ""; \
		echo "Or add to your shell profile:"; \
		echo "  export RUSTC_WRAPPER=sccache"; \
		exit 1; \
	elif [ "$$RUSTC_WRAPPER" != "sccache" ]; then \
		echo "⚠️  RUSTC_WRAPPER is set to '$$RUSTC_WRAPPER' (expected: 'sccache')"; \
		exit 1; \
	else \
		echo "✅ sccache is enabled (RUSTC_WRAPPER=sccache)"; \
		echo ""; \
		echo "Cache directory: ~/.cache/sccache/testcase-manager"; \
		if command -v sccache >/dev/null 2>&1; then \
			echo ""; \
			sccache --show-stats; \
		fi; \
	fi
.PHONY: sccache-check

sccache-clean:
	@sccache --stop-server || true
	@echo "sccache server stopped"
	@echo "Note: Global cache directory preserved at ~/.cache/sccache/testcase-manager"
	@echo "To manually remove cache: rm -rf ~/.cache/sccache/testcase-manager"
.PHONY: sccache-clean

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
	$(MAKE) shellcheck
.PHONY: verify-scripts

shellcheck:
	@echo "Running shellcheck on shell scripts (errors only)..."
	@if ! command -v shellcheck > /dev/null 2>&1; then \
		echo "Warning: shellcheck not installed, skipping"; \
		exit 0; \
	fi; \
	FAILED=0; \
	for script in $$(find scripts tests/integration -type f -name "*.sh" 2>/dev/null); do \
		echo "Checking: $$script"; \
		if shellcheck -S error "$$script" > /dev/null 2>&1; then \
			echo "  ✓ PASSED"; \
		else \
			echo "  ✗ FAILED"; \
			shellcheck -S error "$$script" 2>&1 | head -10; \
			FAILED=1; \
		fi; \
	done; \
	if [ $$FAILED -eq 1 ]; then \
		echo "Some shellcheck validations failed"; \
		exit 1; \
	else \
		echo "All shell scripts pass shellcheck"; \
	fi
.PHONY: shellcheck

test-e2e-validate-yaml: build-validate-yaml
	cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json tests/sample/gsma_4.4.2.2_TC.yml >/dev/null 2>&1
	! cargo run --bin validate-yaml -- --schema schemas/test-case.schema.json tests/sample/data.yml >/dev/null 2>&1
	./tests/integration/test_validate_yaml_multi_e2e.sh
	./tests/integration/test_validate_yaml_watch_e2e.sh
.PHONY: test-e2e-validate-yaml

test-e2e-auto-schema: build
	./tests/integration/test_auto_schema_validation_e2e.sh
.PHONY: test-e2e-auto-schema

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

validate-all-testcases: build-validate-yaml
	SCHEMA_FILE=schemas/test-case.schema.json ./scripts/validate-files.sh --pattern '\.ya?ml$$' --validator ./scripts/validate-yaml-wrapper.sh
.PHONY: validate-all-testcases

verify-testcases: build-validate-yaml
	@echo "Verifying test case files against schema..."
	@FAILED=0; \
	for file in $$(find testcases tests/sample data -type f \( -name "*.yml" -o -name "*.yaml" \) -not \( -path "*/expected_output_reports/*" -o -path "*/testcase_results_container/*" -o -path "*/generated_samples/*" -o -path "*/verifier_scenarios_incorrect/*" -o -name "*te.y*" -o -iname "sample_test_runs.yaml" -o -name "*wrong*" -o -name "data.yml" -o -name "steps-in-json.yml" -o -name "1.yaml" -o -name "SGP.22_4.4.2.yaml" -o -name "conditional_verification_example.yml" -o -name "doc_gen_*.yml" -o -name "*container*" -o -path "*test_case_result*" -o -path "*test_result_01*" \) 2>/dev/null); do \
		echo "Validating: $$file"; \
		if ./target/debug/validate-yaml --schema schemas/test-case.schema.json "$$file" >/dev/null 2>&1; then \
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

# Generate a detailed validation report for all test case files
# This target creates a comprehensive validation report that includes:
# - Pass/fail status for each test case file
# - Detailed error messages for any validation failures
# - Summary statistics (total files, passed count, failed count)
# - Troubleshooting commands for failed validations
# The report is saved to reports/validation_report.txt and displayed to stdout
validate-testcases-report: build-validate-yaml
	@mkdir -p reports
	@uv run python3.14 scripts/generate_validation_report.py
	@echo ""
	@echo "========================================="
	@echo "Displaying Validation Report"
	@echo "========================================="
	@cat reports/validation_report.txt
.PHONY: validate-testcases-report

validate-output-schemas:
	@echo "Validating expected output sample files against schemas..."
	./scripts/validate-output-schemas.sh
.PHONY: validate-output-schemas

validate-envelope-schemas:
	@echo "Validating TCMS envelope schemas..."
	./scripts/validate_envelope_schemas.sh
.PHONY: validate-envelope-schemas

# Generate a JSON report comparing test execution before and after crate splitting
# This target runs cargo tests on both the 'main' and 'split-binaries-into-crates' branches
# and generates a comprehensive comparison report including:
# - Which tests were executed before and after the change
# - After splitting, in which crate is each test located
# - Total execution time before and after with percentage change
# - New, removed, and common tests between the two states
# The report is saved to reports/test_comparison_report.json
test-comparison-report:
	@mkdir -p reports
	@echo "Generating test comparison report..."
	@uv run python3.14 scripts/test_comparison_report.py \
		--run-tests \
		--before-ref main \
		--after-ref split-binaries-into-crates \
		--output reports/test_comparison_report.json \
		--verbose
	@echo ""
	@echo "Report saved to: reports/test_comparison_report.json"
	@echo "View with: cat reports/test_comparison_report.json | jq ."
.PHONY: test-comparison-report

# Generate test comparison report from pre-saved test outputs
# Usage: make test-comparison-from-files BEFORE=before.txt AFTER=after.txt
test-comparison-from-files:
	@mkdir -p reports
	@if [ -z "$(BEFORE)" ] || [ -z "$(AFTER)" ]; then \
		echo "Error: BEFORE and AFTER variables must be set"; \
		echo "Usage: make test-comparison-from-files BEFORE=before.txt AFTER=after.txt"; \
		exit 1; \
	fi
	@echo "Generating test comparison report from saved outputs..."
	@uv run python3.14 scripts/test_comparison_report.py \
		--before $(BEFORE) \
		--after $(AFTER) \
		--output reports/test_comparison_report.json \
		--verbose
	@echo ""
	@echo "Report saved to: reports/test_comparison_report.json"
	@echo "View with: cat reports/test_comparison_report.json | jq ."
.PHONY: test-comparison-from-files

watch: build-validate-yaml
	./scripts/watch-yaml-files.sh
.PHONY: watch

watch-verbose: build-validate-yaml
	SCHEMA_FILE=schemas/test-case.schema.json ./scripts/validate-files.sh --pattern '\.ya?ml$$' --validator ./scripts/validate-yaml-wrapper.sh --watch --verbose
.PHONY: watch-verbose

clean-validation-cache:
	rm -rf .validation-cache/
.PHONY: clean-validation-cache

run-test-executor: build-test-executor
	cargo run --bin test-executor
.PHONY: run-test-executor

test-executor-sample: build-test-executor
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

generate-docs: build
	./scripts/generate_documentation_reports.sh
.PHONY: generate-docs

generate-docs-all: build
	./scripts/generate_documentation_reports.sh --logs-dir testcases --test-case-dir testcases
.PHONY: generate-docs-all

generate-docs-coverage: setup-python-for-test build
	./scripts/generate_documentation_coverage_report.sh
.PHONY: generate-docs-coverage

test-container-compat: build
	./scripts/test_container_yaml_compatibility.sh
.PHONY: test-container-compat

acceptance-test: build-acceptance-binaries
	@echo "========================================="
	@echo "Running Acceptance Test Suite"
	@echo "========================================="
	@echo ""
	@mkdir -p test-acceptance/reports
	@LOG_FILE="test-acceptance/reports/acceptance_suite_execution.log"; \
	echo "Execution log: $$LOG_FILE"; \
	echo ""; \
	if ! command -v test-plan-documentation-generator > /dev/null 2>&1 && [ -z "$$TEST_PLAN_DOC_GEN" ]; then \
		echo "ERROR: test-plan-documentation-generator (TPDG) not found"; \
		echo ""; \
		echo "TPDG is required for acceptance tests."; \
		echo ""; \
		echo "Install options:"; \
		echo "  1. Install globally:"; \
		echo "     cargo install test-plan-documentation-generator"; \
		echo ""; \
		echo "  2. Set TEST_PLAN_DOC_GEN environment variable:"; \
		echo "     export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator"; \
		echo ""; \
		exit 1; \
	fi; \
	if [ -n "$$TEST_PLAN_DOC_GEN" ]; then \
		echo "Using TPDG from: $$TEST_PLAN_DOC_GEN"; \
	else \
		echo "Using TPDG from PATH: $$(which test-plan-documentation-generator)"; \
	fi; \
	echo ""; \
	if ./test-acceptance/run_acceptance_suite.sh 2>&1 | tee "$$LOG_FILE"; then \
		echo ""; \
		echo "=========================================" | tee -a "$$LOG_FILE"; \
		echo "Acceptance Test Suite: SUCCESS" | tee -a "$$LOG_FILE"; \
		echo "=========================================" | tee -a "$$LOG_FILE"; \
		echo "" | tee -a "$$LOG_FILE"; \
		echo "Full execution log: $$LOG_FILE" | tee -a "$$LOG_FILE"; \
		echo "Summary report: test-acceptance/reports/acceptance_suite_summary.txt" | tee -a "$$LOG_FILE"; \
		exit 0; \
	else \
		EXIT_CODE=$$?; \
		echo ""; \
		echo "=========================================" | tee -a "$$LOG_FILE"; \
		echo "Acceptance Test Suite: FAILED" | tee -a "$$LOG_FILE"; \
		echo "=========================================" | tee -a "$$LOG_FILE"; \
		echo "" | tee -a "$$LOG_FILE"; \
		echo "Full execution log: $$LOG_FILE" | tee -a "$$LOG_FILE"; \
		echo "Summary report: test-acceptance/reports/acceptance_suite_summary.txt" | tee -a "$$LOG_FILE"; \
		echo "" | tee -a "$$LOG_FILE"; \
		echo "Review the logs above for details on failures." | tee -a "$$LOG_FILE"; \
		exit 1; \
	fi
.PHONY: acceptance-test

test-e2e-acceptance: build-acceptance-binaries
	@echo "========================================="
	@echo "Running Acceptance Suite E2E Tests"
	@echo "========================================="
	@echo ""
	./test-acceptance/tests/test_acceptance_suite_e2e.sh
.PHONY: test-e2e-acceptance

build-acceptance-binaries:
	@echo "Building required binaries for acceptance tests..."
	@cargo build --package test-executor
	@cargo build --package verifier
	@cargo build --package validate-yaml
	@echo "✓ All required binaries built successfully"
	@echo ""
.PHONY: build-acceptance-binaries

install-loc:
	./scripts/install-loc.sh --local
.PHONY: install-loc

loc:
	./scripts/compute-loc.sh
.PHONY: loc

loc-verbose:
	./scripts/compute-loc.sh --verbose
.PHONY: loc-verbose

loc-json:
	./scripts/compute-loc.sh --format json
.PHONY: loc-json

loc-yaml:
	./scripts/compute-loc.sh --format yaml
.PHONY: loc-yaml

loc-report:
	@mkdir -p reports/loc
	./scripts/compute-loc.sh --output reports/loc/loc_statistics.txt
.PHONY: loc-report

setup-python:
	./scripts/setup_python_env.sh
.PHONY: setup-python

verify-python:
	./scripts/verify_python_env.sh
.PHONY: verify-python
