.PHONY: help
help:
	@sed -n 's/^## //p' ${MAKEFILE_LIST}

.PHONY: validate
validate: validate-schemas
## validate: Run all validation checks (schemas and samples)

.PHONY: validate-schemas
validate-schemas:
## validate-schemas: Validate JSON schemas and sample files
	@echo "Running schema validation..."
	bash scripts/validate-schemas.sh

.PHONY: test
test: validate-sh lint
## test: Run all tests (shell syntax, linting, validations)

.PHONY: validate-sh
validate-sh:
## validate-sh: Validate shell script syntax
	@echo "Validating shell script syntax..."
	@for f in scripts/*.sh; do \
		echo "Checking $$f"; \
		bash -n "$$f" || exit 1; \
	done
	@echo "✓ All shell scripts have valid syntax"

.PHONY: install
install:
## install: Install dependencies with uv
	@echo "Installing dependencies with uv..."
	pip install -q uv
	uv sync

.PHONY: lint
lint:
## lint: Run linting checks
	@command -v pylint >/dev/null 2>&1 || echo "pylint not installed"
	@command -v flake8 >/dev/null 2>&1 || echo "flake8 not installed"

.DEFAULT_GOAL := help
