# ============================================================================
# Projector Build Result Caching Makefile
# ============================================================================
#
# This makefile provides convenient targets for using Projector to cache
# and track Rust compilation results across commits and worktrees.
#
# Projector is a project health tracking tool that maintains a SQLite
# database of build, test, and lint check results per commit SHA.
#
# QUICK START:
#   make build-proj              - Build and log results to projector
#   make test-proj               - Test and log results to projector
#   make proj-status             - View build status across worktrees
#   make proj-report             - Generate build report
#
# DETAILED TARGETS:
#   make proj-build              - Build all crates and log to projector
#   make proj-test               - Run tests and log to projector
#   make proj-lint               - Run clippy and log to projector
#   make proj-release            - Release build and log to projector
#   make proj-all                - Run all checks and log to projector
#
# STATUS/REPORTING:
#   make proj-status             - Show latest status for all worktrees
#   make proj-history            - Show build history for current branch
#   make proj-report             - Generate table report
#   make proj-export-csv         - Export to CSV
#   make proj-export-json        - Export to JSON
#
# ADVANCED:
#   make proj-worktrees          - List configured worktrees
#   make proj-checks             - List configured checks
#   make proj-sync-export        - Export database for team sharing
#   make proj-sync-import        - Import database from team
#   make proj-clean              - Reset database
#

.PHONY: proj-build proj-test proj-lint proj-release proj-all
.PHONY: proj-status proj-history proj-report proj-export-csv proj-export-json
.PHONY: proj-worktrees proj-checks proj-project-info
.PHONY: proj-sync-export proj-sync-import proj-clean
.PHONY: build-proj test-proj lint-proj release-proj
.PHONY: proj-runner-test proj-runner-build proj-runner-lint proj-runner-all proj-runner-custom
.PHONY: proj-full-build proj-workflow proj-ci-build proj-ci-test proj-ci-all
.PHONY: proj-archive proj-help

# Projector executable (use uv if proj not in PATH)
PROJECTOR := $(shell command -v proj >/dev/null 2>&1 && echo proj || echo "uv run --project ~/repos/projector proj")

# Helper script
PROJ_HELPER := ./scripts/log-build-results.sh

# Current worktree (auto-detect from git)
CURRENT_WORKTREE := $(shell git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

# Project name
PROJECT_NAME := testcase-generator

# Runner options (use RUNNER_OPTS=-B to bypass cache)
RUNNER_OPTS ?=

# ============================================================================
# Build Targets with Projector Logging
# ============================================================================

proj-build: ## Build all crates and log results to projector
	@echo "Building with projector logging..."
	@$(PROJ_HELPER) $(CURRENT_WORKTREE) build

proj-test: ## Run tests and log results to projector
	@echo "Testing with projector logging..."
	@$(PROJ_HELPER) $(CURRENT_WORKTREE) test

proj-lint: ## Run clippy linting and log results to projector
	@echo "Linting with projector logging..."
	@$(PROJ_HELPER) $(CURRENT_WORKTREE) lint

proj-release: ## Build release binary and log results to projector
	@echo "Release build with projector logging..."
	@$(PROJ_HELPER) $(CURRENT_WORKTREE) release

proj-all: ## Run all checks (build, test, lint, release) and log to projector
	@echo "Running all checks with projector logging..."
	@$(PROJ_HELPER) $(CURRENT_WORKTREE) all

# Aliases for convenience
build-proj: proj-build
test-proj: proj-test
lint-proj: proj-lint
release-proj: proj-release

# ============================================================================
# Status and Reporting Targets
# ============================================================================

proj-status: ## Show latest build status for all worktrees
	@echo "Build status for $(PROJECT_NAME):"
	@$(PROJECTOR) status $(PROJECT_NAME)

proj-history: ## Show build history for current worktree
	@echo "Build history for $(PROJECT_NAME) / $(CURRENT_WORKTREE):"
	@$(PROJECTOR) status $(PROJECT_NAME) $(CURRENT_WORKTREE)

proj-report: ## Generate table report of all builds
	@echo "Build report for $(PROJECT_NAME):"
	@$(PROJECTOR) report $(PROJECT_NAME)

proj-export-csv: ## Export build results to CSV
	@echo "Exporting to CSV..."
	@$(PROJECTOR) report $(PROJECT_NAME) --format csv > projector-report-$(shell date +%Y%m%d-%H%M%S).csv
	@echo "✓ Exported to projector-report-*.csv"

proj-export-json: ## Export build results to JSON
	@echo "Exporting to JSON..."
	@$(PROJECTOR) report $(PROJECT_NAME) --format json > projector-report-$(shell date +%Y%m%d-%H%M%S).json
	@echo "✓ Exported to projector-report-*.json"

proj-report-since: ## Export report since date (usage: make proj-report-since DATE=2026-05-01)
	@if [ -z "$(DATE)" ]; then \
		echo "Usage: make proj-report-since DATE=2026-05-01"; \
		exit 1; \
	fi
	@echo "Build report since $(DATE):"
	@$(PROJECTOR) report $(PROJECT_NAME) --since $(DATE) --format table

# ============================================================================
# Configuration and Setup
# ============================================================================

proj-worktrees: ## List configured worktrees
	@echo "Worktrees for $(PROJECT_NAME):"
	@$(PROJECTOR) worktree list $(PROJECT_NAME)

proj-checks: ## List configured checks
	@echo "Checks for $(PROJECT_NAME):"
	@$(PROJECTOR) check list $(PROJECT_NAME)

proj-project-info: ## Show project configuration
	@echo "Project info for $(PROJECT_NAME):"
	@$(PROJECTOR) project show $(PROJECT_NAME)

# ============================================================================
# Database Management and Sharing
# ============================================================================

proj-sync-export: ## Export database for team sharing
	@echo "Exporting projector database..."
	@$(PROJECTOR) sync export --output projector-$(shell date +%Y%m%d-%H%M%S).db
	@echo "✓ Database exported - share projector-*.db with team"

proj-sync-import: ## Import database from team (usage: make proj-sync-import FILE=path/to/projector.db)
	@if [ -z "$(FILE)" ]; then \
		echo "Usage: make proj-sync-import FILE=path/to/projector.db"; \
		exit 1; \
	fi
	@echo "Importing projector database from $(FILE)..."
	@$(PROJECTOR) sync import $(FILE)
	@echo "✓ Database imported"

proj-archive: ## Archive database snapshot for git (one-time per milestone)
	@echo "Archiving projector database..."
	@mkdir -p .projector-archive
	@cp .projector.db .projector-archive/projector-$(shell git rev-parse --short HEAD)-$(shell date +%Y%m%d-%H%M%S).db
	@echo "✓ Database archived to .projector-archive/"
	@echo "Run: git add .projector-archive/ && git commit -m 'Archive projector database'"

proj-clean: ## Reset projector database (WARNING: deletes all cached results)
	@echo "⚠ WARNING: This will delete all cached build results!"
	@read -p "Continue? [y/N] " -n 1 -r; \
	echo; \
	if [[ $$REPLY =~ ^[Yy]$$ ]]; then \
		rm -f .projector.db; \
		$(PROJECTOR) init --local; \
		echo "✓ Database reset"; \
	else \
		echo "Cancelled"; \
	fi

# ============================================================================
# Workflow Helpers
# ============================================================================

proj-full-build: ## Complete workflow: build, test, lint + log to projector
	@echo "Running full build workflow..."
	@$(MAKE) proj-build
	@$(MAKE) proj-test
	@$(MAKE) proj-lint
	@echo ""
	@$(MAKE) proj-status

proj-workflow: ## Development workflow: changes → build → test → export results
	@echo "Development workflow..."
	@echo "1. Running checks..."
	@$(MAKE) proj-all
	@echo ""
	@echo "2. Current status:"
	@$(MAKE) proj-status
	@echo ""
	@echo "3. Exporting for team..."
	@$(MAKE) proj-sync-export

# ============================================================================
# Projector Runner Targets (Execute Make Targets via Projector)
# ============================================================================

proj-runner-test: ## Run 'make test' via projector runner (use RUNNER_OPTS=-B to bypass cache)
	@echo "Running 'make test' via projector runner..."
	@$(PROJECTOR) runner -p $(PROJECT_NAME) -w $(CURRENT_WORKTREE) $(RUNNER_OPTS) make test

proj-runner-build: ## Run 'make build' via projector runner (use RUNNER_OPTS=-B to bypass cache)
	@echo "Running 'make build' via projector runner..."
	@$(PROJECTOR) runner -p $(PROJECT_NAME) -w $(CURRENT_WORKTREE) $(RUNNER_OPTS) make build

proj-runner-lint: ## Run 'make lint' via projector runner (use RUNNER_OPTS=-B to bypass cache)
	@echo "Running 'make lint' via projector runner..."
	@$(PROJECTOR) runner -p $(PROJECT_NAME) -w $(CURRENT_WORKTREE) $(RUNNER_OPTS) make lint

proj-runner-all: ## Run all checks via projector runner (use RUNNER_OPTS=-B to bypass cache)
	@echo "Running all checks via projector runner..."
	@$(PROJECTOR) runner -p $(PROJECT_NAME) -w $(CURRENT_WORKTREE) $(RUNNER_OPTS) make build test lint

proj-runner-custom: ## Run custom make target via projector (usage: make proj-runner-custom TARGET=your-target [RUNNER_OPTS=-B])
	@if [ -z "$(TARGET)" ]; then \
		echo "Usage: make proj-runner-custom TARGET=target-name [RUNNER_OPTS=-B]"; \
		exit 1; \
	fi
	@echo "Running 'make $(TARGET)' via projector runner..."
	@$(PROJECTOR) runner -p $(PROJECT_NAME) -w $(CURRENT_WORKTREE) $(RUNNER_OPTS) make $(TARGET)

# ============================================================================
# CI/CD Integration Helpers
# ============================================================================

proj-ci-build: ## CI mode: build and log (for CI/CD pipelines)
	@$(PROJ_HELPER) $(CURRENT_WORKTREE) build
	@exit_code=$$?; \
	if [ $$exit_code -ne 0 ]; then \
		echo "✗ Build failed"; \
		exit 1; \
	fi

proj-ci-test: ## CI mode: test and log (for CI/CD pipelines)
	@$(PROJ_HELPER) $(CURRENT_WORKTREE) test
	@exit_code=$$?; \
	if [ $$exit_code -ne 0 ]; then \
		echo "✗ Tests failed"; \
		exit 1; \
	fi

proj-ci-all: ## CI mode: run all checks and log (for CI/CD pipelines)
	@$(PROJ_HELPER) $(CURRENT_WORKTREE) all

# ============================================================================
# Help and Information
# ============================================================================

proj-help: ## Show projector targets
	@echo "Projector Build Caching Targets:"
	@echo ""
	@echo "Quick Start:"
	@echo "  make build-proj          - Build and log to projector"
	@echo "  make test-proj           - Test and log to projector"
	@echo "  make proj-status         - View build status"
	@echo ""
	@echo "Detailed Build:"
	@echo "  make proj-build          - Build all crates"
	@echo "  make proj-test           - Run tests"
	@echo "  make proj-lint           - Run clippy"
	@echo "  make proj-release        - Release build"
	@echo "  make proj-all            - All checks"
	@echo ""
	@echo "Projector Runner (via proj runner command):"
	@echo "  make proj-runner-test    - Run 'make test' via projector"
	@echo "  make proj-runner-build   - Run 'make build' via projector"
	@echo "  make proj-runner-lint    - Run 'make lint' via projector"
	@echo "  make proj-runner-all     - Run all checks via projector"
	@echo "  make proj-runner-custom TARGET=target - Run custom target via projector"
	@echo ""
	@echo "Cache Control:"
	@echo "  make proj-runner-test                  - Use cache (if available)"
	@echo "  make proj-runner-test RUNNER_OPTS=-B   - Force bypass cache (fresh run)"
	@echo ""
	@echo "Note: Cache is per-commit. New commits invalidate cache automatically."
	@echo ""
	@echo "Reporting:"
	@echo "  make proj-report         - Table report"
	@echo "  make proj-export-csv     - CSV export"
	@echo "  make proj-export-json    - JSON export"
	@echo ""
	@echo "Team Sharing:"
	@echo "  make proj-sync-export    - Export for sharing"
	@echo "  make proj-sync-import FILE=path/to/projector.db"
	@echo "  make proj-archive        - Archive to git"
	@echo ""
	@echo "Configuration:"
	@echo "  make proj-worktrees      - List worktrees"
	@echo "  make proj-checks         - List checks"
	@echo "  make proj-clean          - Reset database"
	@echo ""
	@echo "Workflows:"
	@echo "  make proj-full-build     - Full build + test + lint"
	@echo "  make proj-workflow       - Dev workflow with export"
