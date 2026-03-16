# Projector Usage Guide

This project is configured to work with [Projector](https://github.com/alvarogarcia7/projector), a project automation tool that helps ensure code quality and consistency.

> **Quick Reference**: See [docs/PROJECTOR_QUICK_REF.md](docs/PROJECTOR_QUICK_REF.md) for a condensed command reference.

## What is Projector?

Projector is a tool that automates common project tasks and quality checks. It allows you to define workflows and checks in a configuration file (`.projector.yml`) and run them consistently across different environments.

## Installation

Install projector from the repository:

```bash
# Clone and install projector
git clone https://github.com/alvarogarcia7/projector.git
cd projector
# Follow the installation instructions in the projector README
```

## Configuration

The project includes a `.projector.yml` configuration file that defines:

- **Checks**: Individual quality checks (format, lint, build, test, etc.)
- **Workflows**: Predefined sequences of checks for different scenarios
- **Settings**: Global configuration options
- **Environment**: Environment variables for checks

## Available Checks

All checks use make targets for consistency with the project's build system.

### Required Checks

- `format` - Check code formatting with rustfmt (`make fmt`)
- `lint` - Run clippy linter (`make lint`)
- `build` - Build all binaries (`make build`)
- `test-unit` - Run unit tests (`make test-unit`)
- `test-e2e` - Run end-to-end integration tests (`make test-e2e`)
- `coverage` - Run code coverage analysis (`make coverage`)
- `verify-scripts` - Verify shell script syntax (`make verify-scripts`)

### Optional Checks

- `test-doc` - Run documentation tests (`make test-doc`)
- `shellcheck` - Run shellcheck on shell scripts (`make shellcheck`)
- `validate-testcases` - Validate test case YAML files (`make verify-testcases`)
- `docs` - Generate Rust documentation (`cargo doc --all-features --no-deps`)
- `generate-docs` - Generate documentation reports (`make generate-docs`)

## Available Workflows

### Pre-commit Workflow

Run before committing code:

```bash
projector run pre-commit
```

Includes:
- Code formatting check
- Linting
- Build
- Unit tests
- Script verification

### CI Workflow

Complete CI pipeline (matches GitHub Actions):

```bash
projector run ci
```

Includes:
- All pre-commit checks
- End-to-end tests
- Coverage analysis
- Documentation generation

### Quick Workflow

Fast validation for rapid iteration:

```bash
projector run quick
```

Includes:
- Code formatting check
- Linting
- Build

### Pre-push Workflow

Run before pushing to remote:

```bash
projector run pre-push
```

Includes:
- All pre-commit checks
- End-to-end tests

## Running Individual Checks

Run a single check:

```bash
projector check format
projector check lint
projector check build
projector check test-unit
```

## Running All Checks

Run all required checks:

```bash
projector check --all
```

## Fix Issues Automatically

Some checks support automatic fixing:

```bash
projector fix format  # Auto-format code with rustfmt
```

## Integration with Git Hooks

You can integrate projector with git hooks for automatic validation.

### Automatic Setup

Use the provided script to set up git hooks automatically:

```bash
./scripts/setup-projector-hooks.sh
```

This will create:
- **pre-commit hook**: Runs format, lint, build, unit tests, and script verification
- **pre-push hook**: Runs all pre-commit checks plus e2e tests

The hooks include fallback to direct make commands if projector is not installed.

### Manual Setup

If you prefer to set up hooks manually:

#### Pre-commit Hook

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash
projector run pre-commit
```

Make it executable:

```bash
chmod +x .git/hooks/pre-commit
```

#### Pre-push Hook

Create `.git/hooks/pre-push`:

```bash
#!/bin/bash
projector run pre-push
```

Make it executable:

```bash
chmod +x .git/hooks/pre-push
```

### Bypassing Hooks

To temporarily bypass hooks:

```bash
git commit --no-verify
git push --no-verify
```

## CI/CD Integration

The projector configuration is aligned with the existing CI/CD pipelines:

- **GitHub Actions**: `.github/workflows/workspace.yml` - mirrors the `ci` workflow
- **GitLab CI**: `.gitlab-ci.yml` - extended CI pipeline with additional stages

You can use projector locally to verify changes will pass CI before pushing.

## Troubleshooting

### Check fails locally but passes in CI

Ensure you have the same environment variables set:

```bash
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Dwarnings"
export RUSTDOCFLAGS="-Dwarnings"
export RUST_BACKTRACE=1
```

Or let projector handle it (environment variables are defined in `.projector.yml`).

### Checks take too long

Use the `quick` workflow for rapid iteration:

```bash
projector run quick
```

Run full validation only before committing or pushing.

## Customization

You can customize the projector configuration by editing `.projector.yml`:

- Add new checks
- Create custom workflows
- Modify existing checks
- Adjust settings (fail_fast, parallel, color, verbose)

## Related Documentation

- [AGENTS.md](AGENTS.md) - Build, lint, test, and coverage commands
- [README.md](README.md) - Project overview and features
- [Makefile](Makefile) - Direct make targets (projector uses these)

## Benefits of Using Projector

1. **Consistency**: Same checks run locally and in CI
2. **Speed**: Run only necessary checks during development
3. **Documentation**: Self-documenting workflow definitions
4. **Flexibility**: Easy to add new checks or workflows
5. **Automation**: Integrate with git hooks for automatic validation
