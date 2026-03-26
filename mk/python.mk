# Python-related make targets
# This file contains all Python script execution, testing, and environment setup targets

# Setup Python environment for running tests
# Installs Python 3.14 and syncs dependencies using uv if available
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

# Setup Python development environment
# Runs the setup script to configure Python environment and dependencies
setup-python:
	./scripts/setup_python_env.sh
.PHONY: setup-python

# Verify Python environment configuration
# Checks that the Python environment is properly configured and all dependencies are available
verify-python:
	./scripts/verify_python_env.sh
.PHONY: verify-python
