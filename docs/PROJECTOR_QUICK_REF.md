# Projector Quick Reference

Quick reference for [Projector](https://github.com/alvarogarcia7/projector) commands and workflows for this project.

## Installation

```bash
# Clone and install projector
git clone https://github.com/alvarogarcia7/projector.git
cd projector
# Follow installation instructions in projector README
```

## Setup Git Hooks

```bash
# Automatic setup (recommended)
make setup-projector-hooks

# Or run script directly
./scripts/setup-projector-hooks.sh
```

## Workflows

### Quick Check (Fast)
```bash
projector run quick
```
- Format check
- Linting
- Build

**Use when**: Quick validation during development

### Pre-commit (Standard)
```bash
projector run pre-commit
```
- All quick checks
- Unit tests
- Script verification

**Use when**: Before committing code (auto-runs if git hooks installed)

### Pre-push (Comprehensive)
```bash
projector run pre-push
```
- All pre-commit checks
- End-to-end tests

**Use when**: Before pushing to remote (auto-runs if git hooks installed)

### CI (Complete)
```bash
projector run ci
```
- All pre-push checks
- Coverage analysis
- Documentation generation

**Use when**: Simulating CI pipeline locally

## Individual Checks

### Code Quality
```bash
projector check format          # Check code formatting
projector check lint             # Run clippy
projector check build            # Build project
```

### Testing
```bash
projector check test-unit        # Unit tests
projector check test-e2e         # E2E tests
projector check test-doc         # Doc tests
projector check coverage         # Coverage analysis
```

### Scripts & Validation
```bash
projector check verify-scripts   # Shell script syntax
projector check shellcheck       # Shellcheck analysis
projector check validate-testcases # YAML validation
```

### Documentation
```bash
projector check docs             # Generate Rust docs
projector check generate-docs    # Generate reports
```

## Auto-fix

```bash
projector fix format             # Auto-format code
```

## Run All Checks

```bash
projector check --all
```

## Configuration

Configuration file: `.projector.yml`

Environment variables (auto-set by projector):
- `CARGO_INCREMENTAL=0`
- `RUSTFLAGS=-Dwarnings`
- `RUSTDOCFLAGS=-Dwarnings`
- `RUST_BACKTRACE=1`

## Bypass Git Hooks

```bash
git commit --no-verify           # Skip pre-commit hook
git push --no-verify             # Skip pre-push hook
```

## Comparison with Make

| Task | Projector | Make |
|------|-----------|------|
| Quick check | `projector run quick` | `make fmt && make lint && make build` |
| Pre-commit | `projector run pre-commit` | `make fmt && make lint && make build && make test-unit && make verify-scripts` |
| Full CI | `projector run ci` | `make fmt && make lint && make build && make test-unit && make test-e2e && make coverage && make verify-scripts` |
| Format check | `projector check format` | `make fmt` |
| Format fix | `projector fix format` | `make fmt` |
| Lint | `projector check lint` | `make lint` |
| Build | `projector check build` | `make build` |
| Unit tests | `projector check test-unit` | `make test-unit` |
| E2E tests | `projector check test-e2e` | `make test-e2e` |

## Help

```bash
projector --help                 # General help
projector run --help             # Workflow help
projector check --help           # Check help
```

## Related Documentation

- [PROJECTOR_USAGE.md](../PROJECTOR_USAGE.md) - Detailed usage guide
- [AGENTS.md](../AGENTS.md) - Build and test commands
- [README.md](../README.md) - Project overview
- [.projector.yml](../.projector.yml) - Configuration file
