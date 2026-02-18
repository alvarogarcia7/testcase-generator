# AGENTS.md

## Commands
- **Build**: make build
- **Lint**: make lint
- **Test**: make test
- **Coverage**: make coverage (run unit tests with coverage analysis, 50% threshold)
- **Coverage E2E**: make coverage-e2e (run unit + e2e tests with coverage analysis, 70% threshold)
- **Coverage HTML**: make coverage-html (generate HTML coverage report)
- **Coverage HTML E2E**: make coverage-html-e2e (generate HTML coverage report with e2e tests)
- **Coverage Report**: make coverage-report (display coverage summary)
- **Coverage Report E2E**: make coverage-report-e2e (display coverage summary with e2e tests)
- **Install Coverage Tools**: make install-coverage-tools (install cargo-llvm-cov and related tools)
- **Verify Scripts**: make verify-scripts (verify syntax of all shell scripts)
- **Watch Mode**: make watch (monitors testcases/ for changes and auto-validates)
- **Documentation**: 
  - `make docs-install` - Install MkDocs and required dependencies
  - `make docs-serve` - Serve documentation locally with live reload
  - `make docs-build` - Build documentation site
  - `make docs-build-pdf` - Build documentation with PDF export
  - `make docs-clean` - Clean documentation build artifacts
- **Dev Server**: N/A

You must build, test, lint, and verify coverage before committing

## Binaries

The project includes several binary utilities:

- **json-escape**: A utility that reads from stdin and performs JSON string escaping. Supports a test mode (`--test`) to validate that escaped output is valid JSON when wrapped in quotes, and verbose mode (`--verbose`) for detailed logging.
  - Build: `make build-json-escape`
  - Run: `make run-json-escape` or `cargo run --bin json-escape`
  - Usage: `echo "text" | json-escape`

## Shell Script Compatibility

**MANDATORY**: All shell scripts and generated bash scripts must be compatible with both BSD and GNU variants of command-line tools, and must work with bash 3.2+ (the default on macOS).

### Key Requirements:
- Scripts must work on macOS (BSD) and Linux (GNU) without modification
- Scripts must be compatible with bash 3.2+ (macOS ships with bash 3.2 by default)
- Avoid GNU-specific flags or options that don't exist in BSD variants
- Avoid bash 4.0+ features like associative arrays (`declare -A`)
- Test commands like `sed`, `grep`, `awk`, `find`, etc. must use portable syntax
- When using regex, ensure patterns are compatible with both POSIX and GNU extended regex
- Use POSIX-compliant shell constructs where possible

### Logging Library

**MANDATORY**: All shell scripts must use the centralized logging library for consistent output formatting.

**Location**: `scripts/lib/logger.sh`

**Usage**:
```bash
#!/usr/bin/env bash
set -e

# Get script directory and source logger
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/logger.sh" || exit 1

# Use logging functions
log_info "Informational message"
log_warning "Warning message"
log_error "Error message"
log_debug "Debug message (only shown if VERBOSE=1)"
log_verbose "Verbose message (only shown if VERBOSE=1)"

# Use color-coded test helpers
pass "Test passed"
fail "Test failed"
info "Information"
section "Section Header"
```

**Available Functions**:
- `log_info "message"` - Standard informational message
- `log_warning "message"` - Warning message
- `log_error "message"` - Error message (outputs to stderr)
- `log_debug "message"` - Debug message (only shown when VERBOSE=1)
- `log_verbose "message"` - Verbose message (only shown when VERBOSE=1)
- `pass "message"` - Success message with green checkmark (✓)
- `fail "message"` - Failure message with red X (✗)
- `info "message"` - Info message with blue info symbol (ℹ)
- `section "title"` - Section header with yellow highlighting

**Cleanup Management**:
The logger library also provides cleanup management for temporary files and background processes:
- `setup_cleanup "/path/to/temp/dir"` - Register temporary directory for cleanup
- `register_background_pid $PID` - Register background process for cleanup
- `disable_cleanup` - Disable automatic cleanup (for debugging)
- `enable_cleanup` - Re-enable automatic cleanup

**Benefits**:
- Consistent formatting across all scripts
- Color-coded output for better readability
- Automatic cleanup of temporary resources
- Easy integration with CI/CD pipelines

### Common Pitfalls:
- `grep -P` (Perl regex) is GNU-only - use `sed -n` with capture groups instead
- `sed -r` is GNU-only - use `sed -E` for BSD/macOS compatibility
- `date` formatting differs between BSD and GNU
- `readlink -f` is GNU-only - use alternative methods for BSD
- `declare -A` (associative arrays) requires bash 4.0+ - use eval with dynamic variable names for bash 3.2+

### Testing:
- Test generated scripts on both macOS and Linux when possible
- Use portable regex patterns that work with both implementations
- Verify scripts work with bash 3.2 (default on macOS)
- Verify script syntax using `make verify-scripts`

## Testing Requirements

**MANDATORY**: All agents must run the full test suite before considering any task complete. Testing is a critical step that cannot be skipped.

### Test Execution
- Run tests using: `cargo test --all-features`
- This ensures comprehensive validation across the entire codebase with all feature flags enabled
- Alternative basic test command: `cargo test`

### Test Requirements
- **All tests must pass** before any code changes can be committed
- If tests fail, investigate and fix the failures before proceeding
- Never commit code with failing tests
- Update or add tests as needed when modifying functionality

## Coverage Testing

**MANDATORY**: Code coverage testing is required to ensure comprehensive test coverage across the codebase.

### Installation

Install coverage tools using the provided installation script:

```bash
make install-coverage-tools
```

Or manually install `cargo-llvm-cov`:

```bash
cargo install cargo-llvm-cov
```

For more details on coverage tool installation, see `scripts/README_COVERAGE_TOOLS.md`.

### Coverage Commands

- **Run unit tests with coverage**: `make coverage`
  - Executes unit tests with coverage analysis enabled
  - Minimum threshold: 50% line coverage
  - Excludes: fuzzy.rs, prompts.rs, main_editor.rs

- **Run all tests with coverage (including e2e)**: `make coverage-e2e`
  - Executes unit tests and e2e integration tests with coverage analysis
  - Minimum threshold: 70% line coverage
  - Excludes: fuzzy.rs, prompts.rs, main_editor.rs

- **Generate HTML coverage report**: `make coverage-html`
  - Creates an interactive HTML report showing line-by-line coverage (unit tests only)
  - Opens automatically in your default browser
  - Useful for identifying untested code paths

- **Generate HTML coverage report with e2e**: `make coverage-html-e2e`
  - Creates an interactive HTML report including e2e test coverage
  - Opens automatically in your default browser

- **Display coverage summary**: `make coverage-report`
  - Shows a summary of coverage statistics in the terminal (unit tests only)
  - Provides quick overview of coverage percentages

- **Display coverage summary with e2e**: `make coverage-report-e2e`
  - Shows a summary of coverage statistics including e2e tests
  - Provides quick overview of coverage percentages

### Coverage Exclusions

The following files are excluded from coverage analysis:
- `src/fuzzy.rs` - Interactive fuzzy finder UI components
- `src/prompts.rs` - Interactive prompt UI components
- `src/main_editor.rs` - Main editor binary entry point

### Coverage Requirements

- **Minimum coverage threshold (unit tests)**: 50% line coverage
- **Minimum coverage threshold (unit + e2e tests)**: 70% line coverage
- Coverage must be maintained or improved with each commit
- New code should strive for higher coverage (80%+) when possible
- Review coverage reports to identify critical untested paths

### Pre-Commit Workflow

Before committing any code changes, complete the following steps in order:

1. **Build**: `make build` - Ensure code compiles without errors
2. **Lint**: `make lint` - Fix any style or quality issues
3. **Test**: `make test` - Verify all tests pass
4. **Coverage**: `make coverage-e2e` - Verify coverage meets 70% threshold with e2e tests
5. **Documentation**: Update documentation as needed for user-facing changes

All steps must complete successfully before committing changes.

## Documentation

This project uses MkDocs for documentation with the Material theme.

### Documentation Commands

- **Install dependencies**: `make docs-install`
  - Installs MkDocs and all required plugins and themes
  - Required before first use or after dependency updates

- **Serve locally**: `make docs-serve`
  - Starts a local development server with live reload
  - Access at http://127.0.0.1:8000/
  - Automatically rebuilds on file changes

- **Build site**: `make docs-build`
  - Generates static HTML documentation in `site/` directory
  - Used for deployment and validation

- **Build with PDF**: `make docs-build-pdf`
  - Generates static HTML documentation plus PDF export
  - PDF available in `site/pdf/` directory after build
  - PDF also available as CI/CD artifact

- **Clean artifacts**: `make docs-clean`
  - Removes `site/` directory and build artifacts
  - Useful for fresh builds or troubleshooting

- **Test setup**: `make docs-test`
  - Runs end-to-end tests for MkDocs setup
  - Validates installation, serving, building, PDF generation, and links
  - See `scripts/README_MKDOCS_TEST.md` for detailed test documentation

- **Test with clean install**: `make docs-test-clean`
  - Runs full test suite with clean installation
  - Removes existing virtualenv and rebuilds from scratch

- **Quick test**: `make docs-test-quick`
  - Runs fast validation skipping serve and unit tests
  - Useful for rapid feedback during documentation development

### Automated Deployment

Documentation is automatically built and deployed on every push to the `main` branch:

- **GitLab Pages**: Documentation is deployed to GitLab Pages (URL configured in project settings)
- **GitHub Pages**: Documentation is also deployed to GitHub Pages (URL configured in repository settings)

The CI/CD pipeline handles building and deploying documentation automatically, including PDF generation.

### PDF Documentation

PDF documentation is available in two ways:

1. **Local build**: After running `make docs-build-pdf`, find the PDF in `site/pdf/` directory
2. **CI/CD artifact**: Download PDF from pipeline artifacts after any main branch build

### Documentation Updates

When making user-facing changes, update the relevant documentation:

- Add new features to appropriate documentation pages
- Update examples and usage instructions
- Keep API documentation in sync with code changes
- Review generated documentation locally before committing


<!-- BACKLOG.MD MCP GUIDELINES START -->

<CRITICAL_INSTRUCTION>

## BACKLOG WORKFLOW INSTRUCTIONS

This project uses Backlog.md MCP for all task and project management activities.

**CRITICAL GUIDANCE**

- If your client supports MCP resources, read `backlog://workflow/overview` to understand when and how to use Backlog for this project.
- If your client only supports tools or the above request fails, call `backlog.get_workflow_overview()` tool to load the tool-oriented overview (it lists the matching guide tools).

- **First time working here?** Read the overview resource IMMEDIATELY to learn the workflow
- **Already familiar?** You should have the overview cached ("## Backlog.md Overview (MCP)")
- **When to read it**: BEFORE creating tasks, or when you're unsure whether to track work

These guides cover:
- Decision framework for when to create tasks
- Search-first workflow to avoid duplicates
- Links to detailed guides for task creation, execution, and finalization
- MCP tools reference

You MUST read the overview resource to understand the complete workflow. The information is NOT summarized here.

</CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD MCP GUIDELINES END -->
