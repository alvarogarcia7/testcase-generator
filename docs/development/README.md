# Development

Documentation for developers contributing to or extending Test Case Manager.

## Development Setup

### Building & Testing

```bash
# Build all binaries
make build

# Run tests
make test

# Run linter
make lint

# Verify script syntax
make verify-scripts
```

### Code Quality

- **[Coverage Testing](coverage.md)** - Comprehensive guide to code coverage testing
  - Unit test coverage (50% minimum)
  - E2E test coverage (70% minimum)
  - HTML reports and LCOV export
  - CI/CD integration

### Integration Tests

The project includes comprehensive end-to-end integration tests:

```bash
# Run E2E tests
make test-e2e

# Run all integration tests
make test-e2e-all

# Run all tests (unit + integration)
make test-all
```

## CI/CD Integration

- **[GitLab CI Setup](gitlab-ci-setup.md)** - Configure GitLab CI pipelines
  - Pipeline configuration
  - Test execution
  - Coverage reporting

- **[GitLab CI Examples](gitlab-ci-examples.md)** - Real-world CI/CD examples
  - Example configurations
  - Best practices
  - Integration patterns

## Implementation Guides

- **[Interactive Implementation](interactive-implementation.md)** - Details on the interactive workflow implementation
  - Architecture overview
  - Component design
  - Extension points

## Pre-Commit Workflow

Before committing any changes, complete these steps in order:

1. **Build**: `make build` - Ensure code compiles without errors
2. **Lint**: `make lint` - Fix any style or quality issues
3. **Test**: `make test` - Verify all tests pass
4. **Coverage**: `make coverage-e2e` - Verify coverage meets 70% threshold with e2e tests

All steps must complete successfully before committing changes.

See [AGENTS.md](../../AGENTS.md) for complete development requirements.

## Coverage Requirements

- **Minimum coverage (unit tests)**: 50% line coverage
- **Minimum coverage (unit + e2e tests)**: 70% line coverage (required for commits)
- **Recommended for new code**: 80%+ line coverage
- **Critical paths**: 90%+ coverage

Excluded files: `fuzzy.rs`, `prompts.rs`, `main_editor.rs`

## Shell Script Guidelines

**MANDATORY**: All shell scripts must:
- Be compatible with both BSD and GNU variants
- Work with bash 3.2+ (macOS default)
- Use the centralized logging library (`scripts/lib/logger.sh`)
- Avoid GNU-specific flags
- Pass syntax verification: `make verify-scripts`

See [AGENTS.md](../../AGENTS.md#shell-script-compatibility) for detailed requirements.

## Testing Requirements

**MANDATORY**: All changes must pass the full test suite:

```bash
cargo test --all-features
```

Never commit code with failing tests. Update or add tests when modifying functionality.

## Documentation

When adding features:
1. Update relevant documentation in `docs/`
2. Add examples if applicable
3. Update the main README if user-facing
4. Keep documentation in sync with code changes

## Tools & Utilities

For coverage tools installation and setup:

```bash
make install-coverage-tools
```

See [scripts/README_COVERAGE_TOOLS.md](../../scripts/README_COVERAGE_TOOLS.md) for details.
