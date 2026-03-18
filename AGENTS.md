# AGENTS.md

## Feature Overview

This project is a YAML-based test harness that converts declarative test case definitions into executable bash scripts. Key features include:

- **Declarative Test Cases**: Define test sequences, steps, and expectations in YAML
- **Variable Capture**: Extract values from command output using regex patterns or commands
- **Conditional Verification**: Support for if/then/else logic in verification expressions
- **Prerequisites**: Define manual and automatic prerequisites with verification commands
- **Environment Variables**: Hydration support with required/optional environment variables
- **Test Execution Lifecycle Hooks**: Optional hooks for custom setup, teardown, logging, and resource management at eight different lifecycle points
- **Shell Script Generation**: Generate portable bash 3.2+ compatible scripts from test cases
- **Comprehensive Validation**: Built-in schema validation and test execution verification

### Test Execution Lifecycle Hooks

Hooks provide optional extensibility points throughout the test execution lifecycle. **Hooks are entirely optional** - test cases work perfectly without them. When defined, hooks enable:

- **Custom Setup/Teardown**: Initialize and clean up resources at various lifecycle points
- **Logging and Monitoring**: Track test execution with custom logging at any stage
- **Resource Management**: Create temporary directories, files, and external resources
- **Integration with External Systems**: Connect to databases, APIs, or monitoring systems
- **Context-Aware Operations**: Access test execution context (sequences, steps, variables)
- **Error Handling**: Choose between strict (fail on error) or lenient (continue on error) modes

**Eight Hook Types Available:**
1. `script_start` - Once at script beginning (global initialization)
2. `setup_test` - Once after script_start (test-wide setup)
3. `before_sequence` - Before each test sequence (sequence initialization)
4. `after_sequence` - After each test sequence (sequence cleanup)
5. `before_step` - Before each test step (step preparation)
6. `after_step` - After each test step (step validation)
7. `teardown_test` - Once before script_end (test-wide cleanup)
8. `script_end` - Once at script end (final logging/cleanup)

See the [Hooks](#hooks) section for detailed documentation and examples.

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
- **Install sccache**: make install-sccache (install sccache compilation cache)
- **sccache Stats**: make sccache-stats (display sccache compilation cache statistics)
- **sccache Clean**: make sccache-clean (clear sccache compilation cache)
- **Verify Scripts**: make verify-scripts (verify syntax of all shell scripts)
- **Watch Mode**: make watch (monitors testcases/ for changes and auto-validates)
- **Generate Docs**: make generate-docs (generate documentation reports using test-plan-documentation-generator)
- **Generate Docs All**: make generate-docs-all (generate documentation reports for all test scenarios using test-plan-documentation-generator)
- **Generate Docs Coverage**: make generate-docs-coverage (run documentation generation with tarpaulin coverage analysis)
- **Test Container Compatibility**: make test-container-compat (verify container YAML compatibility with test-plan-doc-gen)
- **Acceptance Tests**: make acceptance-test (run full acceptance test suite with validation, generation, execution, verification, and documentation)
- **Acceptance Suite E2E Tests**: make test-e2e-acceptance (run E2E integration tests for the acceptance suite orchestrator)
- **Install LOC**: make install-loc (install tokei/loc lines of code counter)
- **LOC Statistics**: make loc (compute lines of code statistics for Rust, Python, Shell, and documentation)
- **LOC Verbose**: make loc-verbose (compute lines of code statistics with verbose output)
- **LOC JSON**: make loc-json (compute lines of code statistics in JSON format)
- **LOC YAML**: make loc-yaml (compute lines of code statistics in YAML format)
- **LOC Report**: make loc-report (generate lines of code statistics report to reports/loc/loc_statistics.txt)
- **Setup Python**: make setup-python (install and configure Python 3.14 with uv package manager)
- **Verify Python**: make verify-python (verify Python 3.14 environment is properly configured)
- **Dev Server**: N/A

### Report Generation

All report generation now uses the Rust-based **test-plan-documentation-generator** (tpdg) tool, which generates AsciiDoc, Markdown, and HTML reports from test cases and verification results.

**Python PDF Generation Removed**: The legacy Python-based PDF generation (scripts/generate_verifier_reports.py) has been removed. The reportlab dependency has been removed from pyproject.toml. The only remaining Python dependency is pyyaml, which is required for the convert_verification_to_result_yaml.py script.

**Report Formats Supported**:
- AsciiDoc (.adoc) - Structured documentation format
- Markdown (.md) - GitHub-compatible documentation
- HTML - Generated from AsciiDoc (requires asciidoctor for conversion)

**Installation**:
```bash
# Install test-plan-documentation-generator globally
cargo install test-plan-documentation-generator

# Or use custom path
export TEST_PLAN_DOC_GEN=/path/to/test-plan-documentation-generator/binary
```

**Usage**:
```bash
# Generate documentation reports
make generate-docs          # Verifier scenarios only
make generate-docs-all      # All test cases
```

**Benefits of test-plan-documentation-generator**:
- Better performance and maintainability
- Native integration with the Rust test framework
- Consistent report generation across all test scenarios
- Support for multiple output formats (AsciiDoc, Markdown, HTML)
- No external Python dependencies for report generation
- Schema validation for container YAML compatibility

**Troubleshooting**:
See [Report Generation Documentation](docs/report_generation.md) for detailed installation, configuration, schema compatibility requirements, and troubleshooting steps.

### Python 3.14 Environment Setup

The project requires Python 3.14 for various utility scripts and CI/CD tools. Python 3.14 is managed using the **`uv`** package manager, which provides fast, reliable Python environment management.

#### Quick Start

**Local Setup**:
```bash
# Install and configure Python 3.14 environment
make setup-python

# Verify Python 3.14 is properly configured
make verify-python
```

**Docker Setup**:
The Docker image automatically installs and configures Python 3.14 during build. No manual setup required.

#### Understanding uv Package Manager

**What is uv?**
- Modern, fast Python package and project manager written in Rust
- Replaces pip, pip-tools, virtualenv, and pyenv functionality
- Provides deterministic dependency resolution with lock files
- Handles Python version management (installation and switching)

**Key Benefits**:
- **Speed**: 10-100x faster than pip for dependency resolution and installation
- **Reliability**: Lock files ensure reproducible environments across machines
- **Simplicity**: Single tool for all Python environment needs
- **Version Management**: Automatically downloads and manages Python versions

**Installation**:
```bash
# Install uv package manager
curl -LsSf https://astral.sh/uv/install.sh | sh

# Add uv to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"
```

**Official Documentation**: https://docs.astral.sh/uv/

#### Environment Setup Process

The setup process performs the following steps:

1. **Verify uv Installation**: Checks that `uv` is available in PATH
2. **Sync Dependencies**: Reads `pyproject.toml` and `uv.lock` to install dependencies
3. **Install Python 3.14**: Downloads and installs Python 3.14 if not already present
4. **Set Default Version**: Configures Python 3.14 as the default version for the project
5. **Create Virtual Environment**: Sets up `.venv/` directory with Python 3.14
6. **Re-sync Dependencies**: Ensures all dependencies are installed for Python 3.14
7. **Verify Installation**: Confirms Python 3.14 is working correctly

**Setup Script Location**: `scripts/setup_python_env.sh`

**What Gets Created**:
- `.venv/` - Virtual environment directory containing Python 3.14 and packages
- `uv.lock` - Lock file with pinned dependency versions (if not already present)
- Python binaries available via `uv run` or by activating virtual environment

#### Available Python Commands

After setup, you can use Python 3.14 in several ways:

**1. Via uv run (Recommended)**:
```bash
# Run Python scripts with uv
uv run python3.14 script.py
uv run python3.14 -c "import yaml; print(yaml.__version__)"

# uv automatically uses the project's virtual environment
uv run python script.py  # Uses Python 3.14 from .venv
```

**2. Activate Virtual Environment**:
```bash
# Activate the virtual environment
source .venv/bin/activate

# Now python3.14 points to the virtual environment
python3.14 --version
python3 --version  # Also points to 3.14 in activated venv

# Deactivate when done
deactivate
```

**3. Direct Invocation (Docker Only)**:
```bash
# In Docker, global symlinks are created
python3.14 --version
python3 --version
python --version  # All point to Python 3.14
```

**4. Check Python Version**:
```bash
# Verify Python 3.14 is active
uv run python3.14 --version  # Python 3.14.x
uv python find 3.14          # Shows path to Python 3.14
```

#### Python Dependencies

The project uses minimal Python dependencies defined in `pyproject.toml`:

| Package | Version | Purpose |
|---------|---------|---------|
| `pyyaml` | >=6.0.3 | YAML parsing (convert_verification_to_result_yaml.py) |
| `jsonschema` | >=4.26.0 | JSON schema validation |
| `mypy` | >=1.19.1 | Static type checking for Python scripts |
| `ruff` | >=0.15.6 | Fast Python linting and code formatting |

**Note**: The project previously used `reportlab` for PDF generation, but this has been removed in favor of the Rust-based `test-plan-documentation-generator` tool.

#### Adding New Dependencies

To add a new Python dependency to the project:

**1. Add to pyproject.toml**:
```bash
# Add a new dependency with version constraint
uv add "package-name>=1.0.0"

# Add a development dependency
uv add --dev "pytest>=7.0.0"

# Add with specific version
uv add "requests==2.31.0"
```

**2. Manual Edit (Alternative)**:
```toml
# Edit pyproject.toml manually
[project]
dependencies = [
    "jsonschema>=4.26.0",
    "mypy>=1.19.1",
    "pyyaml>=6.0.3",
    "ruff>=0.15.6",
    "requests>=2.31.0",  # Add your new dependency
]
```

Then sync:
```bash
uv sync
```

**3. Update Lock File**:
```bash
# Regenerate uv.lock with new dependencies
uv lock

# Or sync (which also updates lock file if needed)
uv sync
```

**4. Verify Installation**:
```bash
# Check that the new package is available
uv run python3.14 -c "import requests; print(requests.__version__)"
```

**5. Update Docker Image**:
After adding dependencies, rebuild the Docker image to include them:
```bash
docker build -t your-image-name .
```

**Best Practices**:
- Use version constraints (`>=`, `~=`) rather than exact versions for flexibility
- Run `uv lock` after adding dependencies to update the lock file
- Commit both `pyproject.toml` and `uv.lock` to version control
- Test that dependencies work in both local and Docker environments

#### Removing Dependencies

**1. Remove from pyproject.toml**:
```bash
# Remove a dependency
uv remove package-name
```

**2. Manual Edit (Alternative)**:
```toml
# Edit pyproject.toml and remove the dependency line
[project]
dependencies = [
    "jsonschema>=4.26.0",
    "mypy>=1.19.1",
    "pyyaml>=6.0.3",
    # "removed-package>=1.0.0",  # Remove this line
]
```

Then sync:
```bash
uv sync
```

**3. Clean Virtual Environment (Optional)**:
```bash
# Remove and recreate virtual environment
rm -rf .venv
uv sync
```

#### Upgrading Dependencies

**Update All Dependencies**:
```bash
# Update all dependencies to latest compatible versions
uv lock --upgrade

# Sync to apply updates
uv sync
```

**Update Specific Package**:
```bash
# Update specific package to latest compatible version
uv lock --upgrade-package pyyaml

# Sync to apply
uv sync
```

**Update to Specific Version**:
```bash
# Edit pyproject.toml to change version constraint
# Then sync
uv sync
```

#### Docker Environment

The Docker image automatically sets up Python 3.14 during build:

**Dockerfile Setup Process**:
1. Copies `uv` binary from official `ghcr.io/astral-sh/uv` image
2. Copies `pyproject.toml` and `uv.lock` to container
3. Runs `uv sync --frozen` to install exact locked versions
4. Installs Python 3.14 and sets as default: `uv python install --default 3.14`
5. Creates global symlinks for `python3.14`, `python3`, and `python`
6. Re-syncs dependencies with Python 3.14: `uv sync --frozen --python 3.14`

**Docker Verification**:
```bash
# Verify Python 3.14 is available in container
docker run your-image python3.14 --version
docker run your-image python3 --version
docker run your-image python --version

# All should output: Python 3.14.x
```

**Rebuild After Dependency Changes**:
```bash
# After modifying pyproject.toml or uv.lock
docker build -t your-image-name .
```

#### Verification Process

The verification script (`scripts/verify_python_env.sh`) performs comprehensive checks:

**Local Environment Tests**:
1. ✓ `python3.14` is available in PATH
2. ✓ `python3.14 --version` returns Python 3.14.x
3. ✓ `python3` points to Python 3.14 (optional)
4. ✓ `uv` package manager is available
5. ✓ `uv python find 3.14` locates Python 3.14
6. ✓ `uv run python3.14 --version` works
7. ✓ Required Python packages (pyyaml, jsonschema) are importable

**Docker Environment Tests**:
1. ✓ `python3.14` is available globally
2. ✓ `python3.14 --version` returns Python 3.14.x
3. ✓ `python3` symlink points to Python 3.14
4. ✓ `python` symlink points to Python 3.14
5. ✓ Required Python packages are importable

**Run Verification**:
```bash
# Local environment
make verify-python

# Or run script directly
./scripts/verify_python_env.sh

# Docker environment
docker run your-image make verify-python
```

#### Troubleshooting Python 3.14 Migration

##### Issue: uv not found

**Symptoms**:
```
Error: uv is not installed
```

**Solution**:
```bash
# Install uv package manager
curl -LsSf https://astral.sh/uv/install.sh | sh

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"

# Reload shell configuration
source ~/.bashrc  # or source ~/.zshrc
```

##### Issue: Python 3.14 not found after setup

**Symptoms**:
```
Error: Failed to find Python 3.14 after installation
```

**Solution**:
```bash
# Manually install Python 3.14
uv python install 3.14

# Verify installation
uv python find 3.14

# Re-run setup
make setup-python
```

##### Issue: Module import errors in Python 3.14

**Symptoms**:
```python
ModuleNotFoundError: No module named 'yaml'
```

**Solution**:
```bash
# Re-sync dependencies
uv sync

# Verify packages are installed
uv run python3.14 -c "import yaml; print('OK')"

# If still failing, recreate virtual environment
rm -rf .venv
uv sync
```

##### Issue: Different Python version in virtual environment

**Symptoms**:
```bash
$ source .venv/bin/activate
$ python --version
Python 3.12.0  # Wrong version!
```

**Solution**:
```bash
# Deactivate current environment
deactivate

# Remove virtual environment
rm -rf .venv

# Reinstall with Python 3.14 explicitly
uv sync --python 3.14

# Verify
source .venv/bin/activate
python --version  # Should show Python 3.14.x
```

##### Issue: uv.lock conflicts after git merge

**Symptoms**:
```
Error: Failed to parse uv.lock
Git merge conflict in uv.lock
```

**Solution**:
```bash
# Resolve git conflicts in uv.lock manually, or:

# Regenerate lock file from pyproject.toml
rm uv.lock
uv lock

# Sync to install dependencies
uv sync
```

##### Issue: Dependency resolution conflicts

**Symptoms**:
```
Error: dependency resolution failed
```

**Solution 1 - Check version constraints**:
```bash
# Review pyproject.toml for conflicting version constraints
cat pyproject.toml

# Try loosening version constraints
# Change: package>=2.0.0,<2.1.0
# To:     package>=2.0.0
```

**Solution 2 - Update dependencies**:
```bash
# Update all dependencies to latest compatible versions
uv lock --upgrade
uv sync
```

**Solution 3 - Fresh install**:
```bash
# Remove lock file and virtual environment
rm uv.lock
rm -rf .venv

# Regenerate from scratch
uv lock
uv sync
```

##### Issue: Docker build fails with Python errors

**Symptoms**:
```
ERROR: Failed to sync Python dependencies in Docker
```

**Solution 1 - Verify lock file is committed**:
```bash
# Ensure uv.lock is in git
git add uv.lock
git commit -m "Add uv.lock"

# Rebuild Docker image
docker build -t your-image-name .
```

**Solution 2 - Update Docker base image**:
```bash
# Check Dockerfile uses recent uv version
# Ensure this line is present:
# COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv
```

**Solution 3 - Clean Docker build**:
```bash
# Build without cache
docker build --no-cache -t your-image-name .
```

##### Issue: Permission denied when running uv

**Symptoms**:
```
Permission denied: cannot create .venv
```

**Solution**:
```bash
# Check directory permissions
ls -la .venv

# Remove and recreate with correct permissions
rm -rf .venv
uv sync

# For Docker, ensure correct user/permissions in Dockerfile
```

##### Issue: Old Python version still active

**Symptoms**:
```bash
$ python3 --version
Python 3.12.0  # Old version
```

**Solution 1 - Use uv run**:
```bash
# Use uv run to ensure correct version
uv run python3.14 --version
```

**Solution 2 - Activate virtual environment**:
```bash
source .venv/bin/activate
python3.14 --version
```

**Solution 3 - Update PATH (local development)**:
```bash
# Find Python 3.14 path
PYTHON_314_PATH=$(uv python find 3.14)

# Add to PATH temporarily
export PATH="$(dirname $PYTHON_314_PATH):$PATH"

# Or add to ~/.bashrc for permanent change
```

##### Issue: Scripts fail with Python syntax errors

**Symptoms**:
```python
SyntaxError: invalid syntax
# Using Python 3.14+ features in older Python
```

**Solution**:
```bash
# Verify script is running with Python 3.14
uv run python3.14 script.py

# Check shebang in script
# Should be: #!/usr/bin/env python3.14
# Or use:    #!/usr/bin/env python3
```

##### Issue: Missing dependencies after migration

**Symptoms**:
```
ImportError: cannot import name 'X' from 'package'
```

**Solution**:
```bash
# Check if package version is compatible with Python 3.14
uv tree | grep package-name

# Update to compatible version
uv add "package-name>=compatible-version"
uv sync

# Or update all dependencies
uv lock --upgrade
uv sync
```

##### Issue: CI/CD fails with Python errors

**Symptoms**:
```
CI Error: Python 3.14 not found in CI environment
```

**Solution**:
```bash
# Ensure CI uses proper setup commands
# Add to CI configuration:

# For local CI:
make setup-python
make verify-python

# For Docker-based CI:
# Ensure Docker image is built with Python 3.14 support
```

#### Advanced uv Commands

**Check Installed Packages**:
```bash
# List all installed packages
uv pip list

# Show dependency tree
uv tree

# Show package information
uv pip show pyyaml
```

**Python Version Management**:
```bash
# List installed Python versions
uv python list

# Install specific Python version
uv python install 3.14.1

# Set default Python version for project
uv python pin 3.14
```

**Virtual Environment Management**:
```bash
# Create virtual environment manually
uv venv --python 3.14

# Remove virtual environment
rm -rf .venv

# Recreate from pyproject.toml
uv sync
```

**Lock File Operations**:
```bash
# Generate lock file without installing
uv lock

# Update lock file with latest compatible versions
uv lock --upgrade

# Update specific packages
uv lock --upgrade-package pyyaml --upgrade-package jsonschema

# Check if lock file is up to date
uv lock --check
```

**Debugging**:
```bash
# Verbose output for debugging
uv sync --verbose

# Very verbose output
uv sync -vv

# Show what would be installed without installing
uv sync --dry-run
```

#### Migration Checklist

When migrating existing Python scripts to Python 3.14:

- [ ] Install uv package manager (`curl -LsSf https://astral.sh/uv/install.sh | sh`)
- [ ] Run `make setup-python` to set up Python 3.14
- [ ] Run `make verify-python` to confirm setup
- [ ] Update scripts to use `uv run python3.14` or activate virtual environment
- [ ] Test all Python scripts with Python 3.14
- [ ] Check for deprecated Python features (if migrating from much older versions)
- [ ] Update CI/CD configuration to use Python 3.14
- [ ] Rebuild Docker images with Python 3.14
- [ ] Update documentation to reflect Python 3.14 requirement
- [ ] Commit `pyproject.toml` and `uv.lock` to version control

#### Additional Resources

- **uv Documentation**: https://docs.astral.sh/uv/
- **uv GitHub Repository**: https://github.com/astral-sh/uv
- **Python 3.14 Release Notes**: https://docs.python.org/3.14/whatsnew/3.14.html
- **pyproject.toml Specification**: https://packaging.python.org/en/latest/specifications/pyproject-toml/

#### Getting Help

If you encounter issues not covered in this documentation:

1. Run `make verify-python` to diagnose the problem
2. Check `uv sync --verbose` output for detailed error messages
3. Review `uv.lock` for dependency conflicts
4. Consult uv documentation: https://docs.astral.sh/uv/
5. Check project-specific scripts: `scripts/setup_python_env.sh` and `scripts/verify_python_env.sh`

You must build, test, lint, verify coverage, and run acceptance tests before committing

## Acceptance Test Suite

The acceptance test suite provides comprehensive end-to-end testing of the entire test execution workflow, from YAML validation through to documentation generation.

### Running Acceptance Tests

**Command**: `make acceptance-test`

This target executes the full acceptance test suite, which includes:
1. Building all required binaries (test-executor, verifier, validate-yaml)
2. Validating TPDG (test-plan-documentation-generator) availability
3. Running all acceptance test stages
4. Capturing output to both console and log file
5. Generating final summary report
6. Displaying statistics and results

**Exit Codes**:
- `0` - All tests passed successfully
- `1` - One or more tests failed

**Prerequisites**:
- TPDG must be installed and available:
  - Install globally: `cargo install test-plan-documentation-generator`
  - Or set environment variable: `export TEST_PLAN_DOC_GEN=/path/to/tpdg`

**Output Files**:
- Execution log: `test-acceptance/reports/acceptance_suite_execution.log`
- Summary report: `test-acceptance/reports/acceptance_suite_summary.txt`
- Test results: `test-acceptance/verification_results/`
- Documentation: `test-acceptance/reports/asciidoc/` and `test-acceptance/reports/markdown/`

### Acceptance Test Stages

The acceptance test suite runs six stages:

1. **YAML Validation** - Validates all test case YAMLs against schema
2. **Script Generation** - Generates executable bash scripts from test cases
3. **Test Execution** - Executes all automated tests (skips manual tests by default)
4. **Verification** - Runs verifier on execution logs to generate container YAMLs
5. **Container Validation** - Validates container YAMLs against schema
6. **Documentation** - Generates AsciiDoc and Markdown documentation using TPDG

### Manual Test Suite Execution

For advanced usage, run the acceptance suite script directly:

```bash
# Run with default settings
./test-acceptance/run_acceptance_suite.sh

# Run with verbose output
./test-acceptance/run_acceptance_suite.sh --verbose

# Include manual tests in execution
./test-acceptance/run_acceptance_suite.sh --include-manual

# Skip specific stages
./test-acceptance/run_acceptance_suite.sh --skip-generation
./test-acceptance/run_acceptance_suite.sh --skip-execution
./test-acceptance/run_acceptance_suite.sh --skip-verification
./test-acceptance/run_acceptance_suite.sh --skip-documentation
```

### CI/CD Integration

The `acceptance-test` target is included in the `pre-commit` checks to ensure all code changes pass the full acceptance test suite before commit. This provides comprehensive validation of:
- Code functionality
- Test execution workflow
- Documentation generation
- Schema compliance
- End-to-end integration

### Acceptance Suite E2E Tests

**Command**: `make test-e2e-acceptance`

The acceptance suite E2E tests validate that the `run_acceptance_suite.sh` orchestrator works correctly by running it on a subset of test cases and verifying all stages complete successfully.

**Test Coverage**:
- Validates all 6 stages complete successfully
- Checks expected files are created at each stage (scripts, logs, containers, documentation)
- Validates final report is generated with correct statistics
- Tests all `--skip-*` flags work correctly (generation, execution, verification, documentation)
- Ensures `--verbose` flag increases logging detail
- Verifies error handling for missing dependencies (TPDG not available)
- Tests timeout handling for long-running scripts
- Confirms cleanup of temporary files after completion
- Tests combining multiple `--skip-*` flags

**Test Subset**:
- 5 success scenarios
- 3 failure scenarios
- 2 hook scenarios

**Documentation**: See `test-acceptance/tests/README.md` for detailed information on the E2E test implementation and adding new tests.

## Binaries

The project includes several binary utilities:

- **json-escape**: A utility that reads from stdin and performs JSON string escaping. Supports a test mode (`--test`) to validate that escaped output is valid JSON when wrapped in quotes, and verbose mode (`--verbose`) for detailed logging.
  - Build: `make build-json-escape`
  - Run: `make run-json-escape` or `cargo run --bin json-escape`
  - Usage: `echo "text" | json-escape`

- **test-plan-documentation-generator-compat**: A compatibility checker that verifies container YAML files are compatible with the test-plan-doc-gen tool. Validates schema compliance, generates compatibility reports, and tests against verifier scenarios.
  - Build: `cargo build --bin test-plan-documentation-generator-compat`
  - Run: `cargo run --bin test-plan-documentation-generator-compat -- <command>`
  - Usage: `test-plan-documentation-generator-compat validate container.yaml`
  - Commands: `validate`, `batch`, `test-verifier-scenarios`, `report`
  - Documentation: `docs/TEST_PLAN_DOC_GEN_COMPATIBILITY.md`

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

## Documentation Generation Coverage

The project includes a specialized coverage reporting tool for analyzing code coverage of the documentation generation workflow.

### Coverage Report Generation

**Command**: `make generate-docs-coverage`

This command executes cargo-tarpaulin across all document generation code paths exercised by sample test cases, generating:
- Coverage report showing which functions and branches were executed
- Total coverage percentage for documentation-related modules
- Detailed HTML and JSON reports (optional)

### Modules Tracked

The coverage analysis focuses on:
- `src/lib.rs` - Library exports
- `src/verification.rs` - Verification and report generation
- `src/verification_templates.rs` - Template rendering
- `src/parser.rs` - YAML parsing
- `src/models.rs` - Data models
- `src/bin/verifier.rs` - Verifier binary
- `src/bin/test-plan-documentation-generator-compat.rs` - Documentation generator compatibility

### Usage

**Basic Coverage Report**:
```bash
make generate-docs-coverage
```

**With HTML Report**:
```bash
./scripts/generate_documentation_coverage_report.sh --html
```

**Custom Output Directory**:
```bash
./scripts/generate_documentation_coverage_report.sh --output-dir /path/to/reports
```

### Output Files

Reports are generated in `reports/coverage/documentation/` (default):
- `tarpaulin-report.json` - Coverage data in JSON format
- `coverage_summary.txt` - Human-readable coverage summary
- `coverage_run.log` - Detailed execution log
- `html/` - HTML coverage report (if `--html` flag used)

### Coverage Workflow

The tool automatically:
1. Checks for and installs cargo-tarpaulin if needed
2. Builds verifier and documentation generator binaries
3. Runs sample test scenarios under coverage instrumentation
4. Processes verification logs and container YAML files
5. Generates comprehensive coverage reports
6. Prints total coverage percentage to stdout

### Sample Scenarios

Coverage analysis runs against these test scenarios:
- `TEST_SUCCESS_001` - Successful test execution
- `TEST_FAILED_FIRST_001` - Failed first step scenario
- `TEST_MULTI_SEQ_001` - Multiple sequences scenario

## Hooks

Hooks provide optional extensibility points in the test execution lifecycle, enabling custom setup, teardown, logging, and resource management. **Hooks are entirely optional** - all test cases function normally without defining any hooks.

### Overview

Hooks allow you to inject custom scripts at eight different points in the test execution lifecycle:

1. **script_start** - Executes once at the very beginning of the generated test script
2. **setup_test** - Executes once after script_start, before any test sequences run
3. **before_sequence** - Executes before each test sequence starts
4. **after_sequence** - Executes after each test sequence completes
5. **before_step** - Executes before each test step
6. **after_step** - Executes after each test step completes
7. **teardown_test** - Executes once after all test sequences, before script_end
8. **script_end** - Executes once at the very end of the test script

### Configuration

Hooks are defined in the test case YAML under the `hooks` key:

```yaml
hooks:
  script_start:
    command: "scripts/script_start.sh"
    on_error: "fail"
  setup_test:
    command: "scripts/setup_test.sh"
  before_sequence:
    command: "scripts/before_sequence.sh"
  after_sequence:
    command: "scripts/after_sequence.sh"
  before_step:
    command: "scripts/before_step.sh"
  after_step:
    command: "scripts/after_step.sh"
    on_error: "continue"
  teardown_test:
    command: "scripts/teardown_test.sh"
  script_end:
    command: "scripts/script_end.sh"
```

#### Hook Configuration Fields

- **command** (required): Path to the script or command to execute. Paths are relative to the test case YAML file location, or can be absolute paths.
- **on_error** (optional): Error handling behavior when the hook fails
  - `fail` (default): Test execution stops immediately if the hook fails
  - `continue`: Hook errors are logged but test execution continues

### Available Environment Variables

Hooks have access to the test execution context through environment variables:

#### All Hooks
- Standard environment variables from the test execution context
- Any environment variables defined in the test case's `hydration_vars`

#### before_sequence and after_sequence
- `TEST_SEQUENCE_ID`: The sequence ID (e.g., "1", "2")
- `TEST_SEQUENCE_NAME`: The sequence name

#### before_step and after_step
- `TEST_SEQUENCE_ID`: The sequence ID
- `TEST_STEP_NUMBER`: The step number
- `TEST_STEP_DESCRIPTION`: The step description
- All sequence-scoped variables defined in the test sequence
- All captured variables from previous steps (in after_step)

#### after_step only
- `STEP_EXIT_CODE`: The exit code of the step command
- `COMMAND_OUTPUT`: The output from the step command

### Common Use Cases

#### 1. Test Environment Setup

Create temporary directories and initialize resources:

```yaml
hooks:
  setup_test:
    command: "scripts/setup_environment.sh"
  teardown_test:
    command: "scripts/cleanup_environment.sh"
```

**setup_environment.sh:**
```bash
#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

# Create test workspace
TEST_WORKSPACE="/tmp/test_workspace_$$"
mkdir -p "$TEST_WORKSPACE"
echo "$TEST_WORKSPACE" > /tmp/test_workspace_path.txt

log_info "Created test workspace: $TEST_WORKSPACE"

# Initialize test database
log_info "Initializing test database..."
sqlite3 "$TEST_WORKSPACE/test.db" "CREATE TABLE tests (id INTEGER, name TEXT);"

log_info "Test environment setup complete"
```

**cleanup_environment.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

# Read workspace path
if [ -f /tmp/test_workspace_path.txt ]; then
    TEST_WORKSPACE=$(cat /tmp/test_workspace_path.txt)
    if [ -d "$TEST_WORKSPACE" ]; then
        rm -rf "$TEST_WORKSPACE"
        log_info "Removed test workspace: $TEST_WORKSPACE"
    fi
    rm -f /tmp/test_workspace_path.txt
fi

log_info "Test environment cleanup complete"
```

#### 2. Custom Logging

Track test execution with detailed logging:

```yaml
hooks:
  script_start:
    command: "scripts/log_start.sh"
  before_sequence:
    command: "scripts/log_sequence_start.sh"
  after_step:
    command: "scripts/log_step_result.sh"
  script_end:
    command: "scripts/log_completion.sh"
```

**log_start.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

START_TIME=$(date +%s)
echo "$START_TIME" > /tmp/test_start_time.txt

log_info "========================================="
log_info "Test Execution Started: $(date)"
log_info "========================================="
```

**log_sequence_start.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

SEQUENCE_ID="${TEST_SEQUENCE_ID:-unknown}"
SEQUENCE_NAME="${TEST_SEQUENCE_NAME:-unknown}"

section "Sequence $SEQUENCE_ID: $SEQUENCE_NAME"
log_info "Starting test sequence: $SEQUENCE_NAME (ID: $SEQUENCE_ID)"
```

**log_step_result.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
EXIT_CODE="${STEP_EXIT_CODE:-unknown}"

if [ "$EXIT_CODE" = "0" ]; then
    pass "Step $STEP_NUMBER completed successfully"
else
    fail "Step $STEP_NUMBER failed with exit code: $EXIT_CODE"
fi
```

**log_completion.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

if [ -f /tmp/test_start_time.txt ]; then
    START_TIME=$(cat /tmp/test_start_time.txt)
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))
    
    log_info "========================================="
    log_info "Test Execution Completed: $(date)"
    log_info "Total Duration: ${DURATION}s"
    log_info "========================================="
    
    rm -f /tmp/test_start_time.txt
fi
```

#### 3. Resource Cleanup

Ensure proper cleanup even when tests fail:

```yaml
hooks:
  before_sequence:
    command: "scripts/allocate_resources.sh"
  after_sequence:
    command: "scripts/release_resources.sh"
    on_error: "continue"  # Always try to clean up
```

**allocate_resources.sh:**
```bash
#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

SEQUENCE_ID="${TEST_SEQUENCE_ID:-1}"
RESOURCE_DIR="/tmp/test_resources_seq_${SEQUENCE_ID}"

mkdir -p "$RESOURCE_DIR"
echo "$RESOURCE_DIR" > "/tmp/resource_dir_seq_${SEQUENCE_ID}.txt"

# Allocate test resources
log_info "Allocated resources for sequence $SEQUENCE_ID: $RESOURCE_DIR"

# Create lock file to track active resources
echo "$$" > "$RESOURCE_DIR/lock"
```

**release_resources.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

SEQUENCE_ID="${TEST_SEQUENCE_ID:-1}"
RESOURCE_FILE="/tmp/resource_dir_seq_${SEQUENCE_ID}.txt"

if [ -f "$RESOURCE_FILE" ]; then
    RESOURCE_DIR=$(cat "$RESOURCE_FILE")
    if [ -d "$RESOURCE_DIR" ]; then
        rm -rf "$RESOURCE_DIR"
        log_info "Released resources for sequence $SEQUENCE_ID"
    fi
    rm -f "$RESOURCE_FILE"
fi
```

#### 4. Integration with External Systems

Connect to external monitoring or reporting systems:

```yaml
hooks:
  script_start:
    command: "scripts/notify_test_start.sh"
    on_error: "continue"  # Don't fail if monitoring unavailable
  after_step:
    command: "scripts/report_step_metrics.sh"
    on_error: "continue"
  script_end:
    command: "scripts/notify_test_complete.sh"
    on_error: "continue"
```

**notify_test_start.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

TEST_ID="${TEST_CASE_ID:-unknown}"
MONITORING_URL="${MONITORING_ENDPOINT:-http://localhost:8080/api/tests}"

# Send test start notification
curl -s -X POST "$MONITORING_URL/start" \
    -H "Content-Type: application/json" \
    -d "{\"test_id\":\"$TEST_ID\",\"status\":\"started\",\"timestamp\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}" \
    > /dev/null 2>&1 || log_warning "Failed to notify monitoring system"

log_info "Notified monitoring system: test started"
```

**report_step_metrics.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

STEP_NUMBER="${TEST_STEP_NUMBER:-unknown}"
EXIT_CODE="${STEP_EXIT_CODE:-unknown}"
METRICS_DB="${METRICS_DATABASE:-/tmp/test_metrics.db}"

# Record step metrics to database
if command -v sqlite3 > /dev/null 2>&1; then
    sqlite3 "$METRICS_DB" \
        "INSERT INTO step_metrics (step_number, exit_code, timestamp) \
         VALUES ($STEP_NUMBER, $EXIT_CODE, datetime('now'));" \
        2>/dev/null || log_verbose "Metrics database not available"
fi

log_verbose "Reported metrics for step $STEP_NUMBER"
```

**notify_test_complete.sh:**
```bash
#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../../../scripts/lib/logger.sh" || exit 1

TEST_ID="${TEST_CASE_ID:-unknown}"
MONITORING_URL="${MONITORING_ENDPOINT:-http://localhost:8080/api/tests}"

# Calculate test duration
if [ -f /tmp/test_start_time.txt ]; then
    START_TIME=$(cat /tmp/test_start_time.txt)
    END_TIME=$(date +%s)
    DURATION=$((END_TIME - START_TIME))
else
    DURATION=0
fi

# Send completion notification
curl -s -X POST "$MONITORING_URL/complete" \
    -H "Content-Type: application/json" \
    -d "{\"test_id\":\"$TEST_ID\",\"status\":\"completed\",\"duration\":$DURATION,\"timestamp\":\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}" \
    > /dev/null 2>&1 || log_warning "Failed to notify monitoring system"

log_info "Notified monitoring system: test completed (${DURATION}s)"
```

### Best Practices

1. **Use the Logger Library**: All hook scripts should use the centralized `scripts/lib/logger.sh` for consistent output formatting
2. **Handle Errors Gracefully**: Use `on_error: "continue"` for cleanup hooks to ensure they always run
3. **Shell Compatibility**: Hook scripts must be compatible with bash 3.2+ (BSD and GNU variants)
4. **Resource Tracking**: Use temporary files to track resources created by hooks for proper cleanup
5. **Minimal Side Effects**: Hooks should be lightweight and not significantly impact test execution time
6. **Environment Variable Access**: Use `${VAR:-default}` syntax to provide defaults for optional variables
7. **Idempotent Operations**: Design hooks to be safely re-runnable when possible

### Example: Complete Test Case with Hooks

See `testcases/examples/hooks/TC_HOOKS_001.yaml` for a comprehensive example demonstrating all eight hook types with:
- Resource management (temporary directories)
- Logging integration (centralized logger library)
- Variable access (sequence and step context)
- Error handling (both fail and continue modes)
- Timing and duration tracking

Full documentation and example hook scripts are available in `testcases/examples/hooks/README.md`.

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

All steps must complete successfully before committing changes.


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
