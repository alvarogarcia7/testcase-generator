# Coverage Tools Installation

This directory contains scripts for installing and managing code coverage tools across different environments.

## Quick Start

### Local Development

Install coverage tools for local development using the Makefile:

```bash
make install-coverage-tools
```

Or run the script directly:

```bash
./scripts/install-coverage-tools.sh --local
```

Or simply run without arguments to auto-detect your environment:

```bash
./scripts/install-coverage-tools.sh
```

### Install All Tools

To install coverage tools for all environments (local, GitHub Actions, GitLab CI):

```bash
./scripts/install-coverage-tools.sh --all
```

## Available Scripts

### `install-coverage-tools.sh`

Comprehensive installation script for code coverage tools across different environments.

**Features:**
- Auto-detects CI environment (GitHub Actions, GitLab CI, or local)
- Installs `cargo-llvm-cov` for local development and GitHub Actions
- Installs `grcov` for GitLab CI
- Installs required Rust components (`llvm-tools-preview`)
- Provides post-installation usage examples
- Interactive prompts for reinstallation/updates

**Usage:**
```bash
./scripts/install-coverage-tools.sh [OPTIONS]
```

**Options:**
- `--local` - Install tools for local development (cargo-llvm-cov)
- `--github` - Install tools for GitHub Actions (cargo-llvm-cov)
- `--gitlab` - Install tools for GitLab CI (grcov)
- `--all` - Install all tools
- `--help` - Show help message

**Examples:**
```bash
# Install for local development (default)
./scripts/install-coverage-tools.sh

# Install for specific environment
./scripts/install-coverage-tools.sh --github
./scripts/install-coverage-tools.sh --gitlab

# Install all tools at once
./scripts/install-coverage-tools.sh --all

# Show help
./scripts/install-coverage-tools.sh --help
```

## Coverage Tools

### cargo-llvm-cov

Modern coverage tool using LLVM's native coverage instrumentation.

**Used in:**
- Local development
- GitHub Actions

**Features:**
- Fast and accurate coverage measurement
- Multiple output formats (LCOV, HTML, JSON)
- Coverage thresholds support
- Integration with Codecov and Coveralls

**Usage:**
```bash
# Run tests with coverage
cargo llvm-cov --all-features --workspace

# Generate HTML report
cargo llvm-cov --all-features --workspace --html --open

# Generate LCOV report for CI
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Fail if coverage is below 70%
cargo llvm-cov --all-features --workspace --fail-under-lines 70
```

**Makefile targets:**
```bash
make coverage          # Run with 70% threshold
make coverage-html     # Generate HTML report
make coverage-report   # Show coverage summary
make coverage-lcov     # Generate LCOV report
```

### grcov

Mozilla's coverage aggregation tool for processing raw coverage data.

**Used in:**
- GitLab CI

**Features:**
- Aggregates coverage from multiple test runs
- Multiple output formats (LCOV, Cobertura, HTML)
- Efficient processing of large codebases

**Usage in GitLab CI:**
```yaml
# .gitlab-ci.yml configuration
rust:build-test-lint:
  script:
    - export CARGO_INCREMENTAL=0
    - export RUSTFLAGS="-Cinstrument-coverage"
    - export LLVM_PROFILE_FILE="coverage-%p-%m.profraw"
    - cargo test --all --all-features --tests
    - grcov . --binary-path ./target/debug/ -s . -t lcov --branch --ignore-not-existing --ignore "/*" -o lcov.info
```

## CI/CD Integration

### GitHub Actions

The repository includes `.github/workflows/coverage.yml` which:
- Automatically installs `cargo-llvm-cov` using `taiki-e/install-action`
- Runs tests with coverage
- Generates LCOV reports
- Uploads to Codecov
- Posts coverage summary as PR comment

**No manual installation needed** - the workflow handles everything.

### GitLab CI

The repository includes `.gitlab-ci.yml` which:
- Downloads pre-built `grcov` binary for faster CI builds
- Runs tests with coverage instrumentation
- Generates coverage reports
- Posts coverage comparison as MR comment

**No manual installation needed** - the pipeline handles everything.

### Local Development

For local development, you must manually install the coverage tools:

```bash
./scripts/install-coverage-tools.sh --local
```

This installs:
- `cargo-llvm-cov` - Main coverage tool
- `llvm-tools-preview` - Required Rust component

## Coverage Requirements

**Minimum coverage threshold:** 70% line coverage

This threshold is enforced in:
- Local development: `make coverage` (via `cargo llvm-cov --fail-under-lines 70`)
- GitHub Actions: `.github/workflows/coverage.yml`
- GitLab CI: `.gitlab-ci.yml`

## Troubleshooting

### cargo-llvm-cov not found

```bash
# Reinstall cargo-llvm-cov
cargo install cargo-llvm-cov

# Verify installation
cargo llvm-cov --version
```

### llvm-tools-preview component missing

```bash
# Install the component
rustup component add llvm-tools-preview

# Verify installation
rustup component list | grep llvm-tools-preview
```

### grcov not found (GitLab CI)

The GitLab CI pipeline should automatically download grcov. If it fails:

```bash
# Check the download URL in .gitlab-ci.yml
# Verify network connectivity in CI environment
# Check for rate limiting on GitHub releases
```

### Coverage reports not generated

```bash
# Clean previous coverage data
cargo llvm-cov clean --workspace

# Run coverage again
cargo llvm-cov --all-features --workspace
```

## Additional Resources

- [cargo-llvm-cov documentation](https://github.com/taiki-e/cargo-llvm-cov)
- [grcov documentation](https://github.com/mozilla/grcov)
- [Rust coverage instrumentation](https://doc.rust-lang.org/rustc/instrument-coverage.html)
- [Codecov documentation](https://docs.codecov.com/)
- Project AGENTS.md - Coverage testing requirements
